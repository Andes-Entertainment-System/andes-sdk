use anyhow::anyhow;
use clap::{arg, Command};
use std::{
    env,
    fs::File,
    io::{self, Read},
    path::Path,
    process, time,
    time::Instant,
};

mod resources;

fn build(project_path: &Path) -> anyhow::Result<()> {
    resources::compile_all(project_path)?;

    let cmd = process::Command::new("cmake")
        .current_dir(project_path)
        .arg(".")
        .stdout(process::Stdio::inherit())
        .stderr(process::Stdio::inherit())
        .spawn()?;

    if !cmd.wait_with_output()?.status.success() {
        return Err(anyhow!("CMake configuration failed"));
    }

    let cmd = process::Command::new("make")
        .current_dir(project_path)
        .stdout(process::Stdio::inherit())
        .stderr(process::Stdio::inherit())
        .spawn()?;

    if !cmd.wait_with_output()?.status.success() {
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
        .subcommand(
            Command::new("build")
                .about("placeholder")
                .arg(arg!([DIRECTORY])),
        )
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
            let default_path = env::current_dir().unwrap_or(Path::new("").to_path_buf());
            let project_path_str: Option<&String> = sub_matches.get_one("DIRECTORY");

            let result: anyhow::Result<()>;
            let start_time = Instant::now();

            match project_path_str {
                Some(x) => result = build(&Path::new(x)),
                None => result = build(&default_path),
            }

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
