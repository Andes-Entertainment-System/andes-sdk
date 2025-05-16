use anyhow::{anyhow, Context};
use clap::{arg, Command};
use std::{
    env,
    fs::{self, File},
    io::{self, Read},
    path::Path,
    process,
    time::{self, Instant},
};

pub mod resources;
pub mod utils;

fn build(project_path: &Path, target: &String) -> anyhow::Result<()> {
    resources::compile_all(project_path)?;

    let build_residual_dir = &project_path.join(".build-residual");
    if !fs::exists(build_residual_dir)? {
        fs::create_dir(build_residual_dir)?;
    }

    // set up cmake
    let cmd = process::Command::new("cmake")
        .current_dir(build_residual_dir)
        .arg("..")
        .arg(format!("-DTARGET={}", target))
        .arg("-GNinja")
        .stdout(process::Stdio::inherit())
        .stderr(process::Stdio::inherit())
        .spawn()?;

    if !cmd.wait_with_output()?.status.success() {
        return Err(anyhow!("CMake configuration failed"));
    }

    // build app
    let cmd = process::Command::new("cmake")
        .current_dir(build_residual_dir)
        .arg("--build")
        .arg(".")
        .stdout(process::Stdio::inherit())
        .stderr(process::Stdio::inherit())
        .spawn()?;

    if !cmd.wait_with_output()?.status.success() {
        return Err(anyhow!("App build failed"));
    }

    // copy app file to build folder
    fs::copy(
        build_residual_dir.join("app.bin"),
        project_path.join("build/app.bin"),
    )
    .context("Failed to copy app binary file to build folder")?;

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

fn sideload(file_path: &Path) -> anyhow::Result<()> {
    let mut port = serialport::new("/dev/ttyUSB0", 230400)
        .timeout(time::Duration::from_millis(10))
        .dtr_on_open(true)
        .preserve_dtr_on_open()
        .open()?;

    let mut app_file = File::open(file_path)?;
    let mut data: Vec<u8> = vec![0; app_file.metadata()?.len() as usize];
    app_file.read(&mut data)?;

    port.write(&(data.len() as u32).to_le_bytes())?; // file size
    port.write(&data)?; // the actual file

    Ok(())
}

fn main() {
    let command = Command::new("andk")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("build").about("placeholder").args(&[
            arg!([DIRECTORY]),
            arg!(-t --target <TARGET>).default_value("wasm"),
        ]))
        .subcommand(
            Command::new("sideload")
                .about("placeholder")
                .arg(arg!([FILE]))
                .arg(arg!([PORT])),
        )
        .subcommand(Command::new("new").about("placeholder"));
    let matches = command.get_matches();

    match matches.subcommand() {
        Some(("build", sub_matches)) => {
            let project_path = match sub_matches.get_one::<String>("DIRECTORY") {
                Some(x) => Path::new(x),
                None => &env::current_dir().unwrap(),
            };
            let target: &String = match sub_matches.get_one::<String>("target") {
                Some(x) => x,
                None => &String::from("wasm"),
            };

            println!("{}", target);

            let start_time = Instant::now();
            let result = build(project_path, target);

            match result {
                Err(x) => println!("BUILD ABORTED: {}", x),
                Ok(_) => println!(
                    "BUILD SUCCESSFUL: took {} seconds",
                    start_time.elapsed().as_millis() as f64 / 1000.0
                ),
            }
        }
        Some(("sideload", sub_matches)) => {
            let default_path = env::current_dir().unwrap_or(Path::new("").to_path_buf());
            let file_path_str: Option<&String> = sub_matches.get_one("FILE");

            let result: anyhow::Result<()>;
            let start_time = Instant::now();

            match file_path_str {
                Some(x) => result = sideload(&Path::new(x)),
                None => result = sideload(&default_path.join("build/out.bin")),
            }

            match result {
                Err(x) => println!("SIDELOAD ABORTED: {}", x),
                Ok(_) => println!(
                    "SIDELOAD SUCCESSFUL: took {} seconds",
                    start_time.elapsed().as_millis() as f64 / 1000.0
                ),
            }
        }
        _ => unreachable!(),
    }
}
