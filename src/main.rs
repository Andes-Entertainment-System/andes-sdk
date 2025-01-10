use anyhow::anyhow;
use clap::{arg, Command};
use std::{env, fs::File, io, path::Path, process, time::Instant};

mod resources;

fn build(project_path: &Path) -> anyhow::Result<()> {
    resources::compile_all(project_path)?;

    let output = process::Command::new("cmake")
        .current_dir(project_path)
        .arg(".")
        .output()
        .expect("Fucked!");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));

    if !output.status.success() {
        return Err(anyhow!("CMake configuration failed"));
    }

    let output = process::Command::new("make")
        .current_dir(project_path)
        .output()
        .expect("Fucked!");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));

    if !output.status.success() {
        return Err(anyhow!("App build failed"));
    }

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
        .subcommand(Command::new("build").about("fuck").arg(arg!([DIRECTORY])))
        .subcommand(Command::new("new").about("shit"));
    let matches = command.get_matches();

    match matches.subcommand() {
        Some(("build", sub_matches)) => {
            let default_path = env::current_dir().unwrap_or(Path::new("").to_path_buf());
            let project_path_str: Option<&String> = sub_matches.get_one("DIRECTORY");

            let build_result: anyhow::Result<()>;
            let build_time = Instant::now();

            match project_path_str {
                Some(x) => build_result = build(&Path::new(x)),
                None => build_result = build(&default_path),
            }

            match build_result {
                Err(x) => println!("BUILD ABORTED: {}", x),
                Ok(_) => println!(
                    "BUILD SUCCESSFUL: took {} seconds",
                    build_time.elapsed().as_millis() as f64 / 1000.0
                ),
            }
        }
        _ => unreachable!(),
    }
}
