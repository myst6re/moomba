#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use simplelog::{CombinedLogger, TermLogger, WriteLogger, LevelFilter, TerminalMode, ColorChoice};
use std::fs::File;
use log::{info, error};
use std::process::{Command, Stdio};
use std::path::{Path, PathBuf};
use same_file::is_same_file;

fn main() -> std::io::Result<()> {
    match CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Debug, simplelog::Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, simplelog::Config::default(), File::create("launcher.log").unwrap()),
        ]
    ) {
        _ => ()
    }
    let current_exe = std::env::current_exe()?;
    let path = match std::env::args().nth(1) {
        Some(ff8_path) => PathBuf::from(ff8_path),
        None => {
            let mut exe_path = current_exe.clone();
            exe_path.pop();
            let config_path = exe_path.join("moomba_path.txt");
            let path = std::fs::read_to_string(config_path);
            match path {
                Ok(path) => PathBuf::from(path.trim()),
                Err(e) => {
                    error!("Error reading link: {}", e);
                    exe_path.join("FF8_Launcher_Original.exe")
                }
            }
        }
    };
    let dir = match path.parent() {
        Some(dir) => dir,
        None => Path::new(".")
    };
    info!("Path={} Dir={}", &path.to_str().unwrap(), &dir.to_str().unwrap());

    if is_same_file(&path, current_exe)? {
        error!("Target exe is the current exe itself");
        Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Target exe is the current exe itself"))
    } else {
        let mut child = Command::new(&path)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .current_dir(dir)
            .spawn()?;
        child.wait()?;
        Ok(())
    }
}
