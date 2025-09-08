use anyhow::Context;
use clap::{arg, Command};
use std::{
    env,
    fs::{self, File},
    io,
    path::{Path, PathBuf},
    process,
    str::FromStr,
    time::Instant,
};
use thiserror::Error;

pub mod metadata;
pub mod resources;
pub mod utils;

#[derive(Clone, Copy, strum_macros::Display, strum_macros::EnumString)]
pub enum BuildTarget {
    Invalid,
    #[strum(ascii_case_insensitive)]
    Wasm,
    #[strum(ascii_case_insensitive)]
    Xtensa,
}

#[derive(Error, Debug)]
pub enum BuildError {
    #[error("The ANDES_SDK_DIR environment variable is not defined")]
    AndesSdkDirNotDefined,
    #[error("Cannot find wasi-sdk. Make sure the WASI_SDK_DIR environment variable points to its directory.")]
    WasiSdkNotFound,
    #[error("Cannot find ESP-IDF. Make sure you're building under an ESP-IDF environment. If you installed 
it with the Visual Studio Code ESP-IDF extension and you're currently using VSCode to build your project, you can open 
a terminal under the ESP-IDF environment by opening the Command Prompt (F1) and looking for \"Open ESP-IDF Terminal\". 
Otherwise, you may look at the Installation page in the ESP-IDF docs for further instructions.")]
    EspIdfNotFound,
    #[error(
        "No target has been set for this project. Make sure to run `andk set-target {{TARGET}}`"
    )]
    NoTargetSet,
    #[error("The specified target is not a valid build target. Valid build targets: wasm, xtensa")]
    InvalidTarget,
    #[error("CMake configuration failed")]
    CMakeConfigFailed,
    #[error("App make failed")]
    MakeFailed,
}

fn timed_task<F: Fn() -> anyhow::Result<()>>(task: F, name: &str) {
    let start_time = Instant::now();

    match task() {
        Err(x) => {
            if env::var("RUST_BACKTRACE").is_ok() {
                println!("{} ABORTED: {}\n{}", name, x, x.backtrace());
            } else {
                println!("{} ABORTED: {}", name, x);
            }
        }
        Ok(_) => println!(
            "{} SUCCESSFUL: took {} seconds",
            name,
            start_time.elapsed().as_millis() as f64 / 1000.0
        ),
    }
}

fn get_target_args(target: BuildTarget) -> anyhow::Result<Vec<String>> {
    return match target {
        BuildTarget::Wasm => {
            let wasi_sdk_path = match env::var("WASI_SDK_DIR") {
                Ok(x) => Ok(PathBuf::from(x)),
                Err(_) => Err(BuildError::WasiSdkNotFound),
            }?;
            println!(
                "-DCMAKE_TOOLCHAIN_FILE={}",
                wasi_sdk_path
                    .join("share/cmake/wasi-sdk.cmake")
                    .to_string_lossy()
            );
            Ok(vec![
                format!(
                    "-DCMAKE_TOOLCHAIN_FILE={}",
                    wasi_sdk_path
                        .join("share/cmake/wasi-sdk.cmake")
                        .to_string_lossy()
                ),
                format!(
                    "-DCMAKE_SYSROOT={}",
                    wasi_sdk_path.join("share/wasi-sysroot").to_string_lossy()
                ),
            ])
        }
        BuildTarget::Xtensa => {
            let idf_path = match env::var("IDF_PATH") {
                Ok(x) => Ok(PathBuf::from(x)),
                Err(_) => Err(BuildError::EspIdfNotFound),
            }?;
            Ok(vec![format!(
                "-DCMAKE_TOOLCHAIN_FILE={}",
                idf_path
                    .join("tools/cmake/toolchain-esp32s3.cmake")
                    .to_string_lossy()
            )])
        }
        _ => Err(BuildError::InvalidTarget.into()),
    };
}

fn clean(project_path: &Path) -> anyhow::Result<()> {
    let build_residual_dir = &project_path.join(".build-residual");
    if fs::exists(build_residual_dir)? {
        fs::remove_dir_all(build_residual_dir)?;
    }

    Ok(())
}

fn set_target(project_path: &Path, target: BuildTarget) -> anyhow::Result<()> {
    clean(project_path)?;

    let build_residual_dir = &project_path.join(".build-residual");
    fs::create_dir(build_residual_dir)?;

    let cmd = process::Command::new("cmake")
        .current_dir(build_residual_dir)
        .arg("..")
        .arg(format!("-DTARGET={}", target))
        .args(get_target_args(target)?)
        .stdout(process::Stdio::inherit())
        .stderr(process::Stdio::inherit())
        .spawn()?;

    if !cmd.wait_with_output()?.status.success() {
        return Err(BuildError::CMakeConfigFailed.into());
    }

    Ok(())
}

fn build(project_path: &Path) -> anyhow::Result<()> {
    // remove (if exists) and (re)create build folder
    let build_path = project_path.join("build");
    if fs::exists(&build_path)? {
        fs::remove_dir_all(&build_path)?;
    }
    let _ = fs::create_dir(build_path);

    resources::compile_all(project_path)?;

    let build_residual_dir = &project_path.join(".build-residual");
    if !fs::exists(build_residual_dir)? {
        return Err(BuildError::NoTargetSet.into());
    }

    // set up cmake
    let cmd = process::Command::new("cmake")
        .current_dir(build_residual_dir)
        .arg("..")
        .stdout(process::Stdio::inherit())
        .stderr(process::Stdio::inherit())
        .spawn()?;

    if !cmd.wait_with_output()?.status.success() {
        return Err(BuildError::CMakeConfigFailed.into());
    }

    // build app
    let cmd = process::Command::new("make")
        .current_dir(build_residual_dir)
        .stdout(process::Stdio::inherit())
        .stderr(process::Stdio::inherit())
        .spawn()?;

    if !cmd.wait_with_output()?.status.success() {
        return Err(BuildError::MakeFailed.into());
    }

    // copy app file to build folder
    fs::copy(
        build_residual_dir.join("app.bin"),
        project_path.join("build/app.bin"),
    )
    .context("Failed to copy app binary file to build folder")?;

    // merge resources.bin and app.bin into out.bin
    let mut out_file = File::create(project_path.join("build/out.bin"))?;

    io::copy(
        &mut File::open(project_path.join("build/resources.bin"))?,
        &mut out_file,
    )?;
    io::copy(
        &mut File::open(project_path.join("build/app.bin"))?,
        &mut out_file,
    )?;

    Ok(())
}

fn main() {
    let command = Command::new("andk")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("clean")
                .about("placeholder")
                .arg(arg!([DIRECTORY])),
        )
        .subcommand(
            Command::new("set-target")
                .about("placeholder")
                .arg(arg!([TARGET]))
                .arg(arg!([DIRECTORY])),
        )
        .subcommand(
            Command::new("build")
                .about("placeholder")
                .arg(arg!([DIRECTORY]))
                .arg(arg!(-t --target <TARGET>).default_value("wasm")),
        )
        .subcommand(Command::new("new").about("placeholder"));
    let matches = command.get_matches();

    match matches.subcommand() {
        Some(("clean", sub_matches)) => {
            let project_path = match sub_matches.get_one::<String>("DIRECTORY") {
                Some(x) => Path::new(x),
                None => &env::current_dir().unwrap(),
            };

            timed_task(|| clean(project_path), "CLEAN");
        }
        Some(("set-target", sub_matches)) => {
            let project_path = match sub_matches.get_one::<String>("DIRECTORY") {
                Some(x) => Path::new(x),
                None => &env::current_dir().unwrap(),
            };
            let target_str: &String = match sub_matches.get_one::<String>("TARGET") {
                Some(x) => x,
                None => &String::from("wasm"),
            };
            let target = BuildTarget::from_str(target_str).unwrap_or(BuildTarget::Invalid);

            timed_task(|| set_target(project_path, target), "SET_TARGET");
        }
        Some(("build", sub_matches)) => {
            let project_path = match sub_matches.get_one::<String>("DIRECTORY") {
                Some(x) => Path::new(x),
                None => &env::current_dir().unwrap(),
            };

            timed_task(|| build(project_path), "BUILD");
        }
        _ => unreachable!(),
    }
}
