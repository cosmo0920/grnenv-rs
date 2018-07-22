use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use std::process;
use std::process::{Command, Stdio};

use clap::ArgMatches;
use kuchiki::traits::*;
use reqwest::{Client, Url};
use reqwest::Response;
use reqwest::Result as ReqwestResult;
use util;

pub fn versions() {
    let groonga_versioned_dir = util::obtain_groonga_versioned_path();
    let paths = fs::read_dir(&Path::new(&groonga_versioned_dir)).unwrap();

    let names = paths.filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path()
                    .file_name()
                    .and_then(|n| n.to_str().map(|s| String::from(s)))
            })
        })
        .collect::<Vec<String>>();

    println!("Installed Groonga:");
    println!("\tsystem");
    for entry in names {
        let e = entry.split("-").collect::<Vec<_>>();
        println!("\t{} ({})",
                 e.get(1).unwrap_or(&""),
                 e.get(2).unwrap_or(&"build from source"));
    }
}

pub fn execute_external_command(cmd: &str, ext_m: &ArgMatches) {
    let command_exe = format!("grnenv-{}{}", cmd, env::consts::EXE_SUFFIX);
    let ext_args: Vec<&str> = match ext_m.values_of("") {
        Some(c) => c.collect(),
        None => vec![],
    };
    let path = search_directories()
        .iter()
        .map(|dir| dir.join(&command_exe))
        .find(|file| is_executable(file));
    let command = match path {
        Some(command) => command,
        None => {
            println!("no such subcommand: `{}`", cmd);
            process::exit(1);
        }
    };
    let err = match Command::new(command)
        .args(&ext_args.as_slice()[0..])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn() {
        Ok(_) => return (),
        Err(e) => e,
    };

    println!("Failed to execute external subcommand. reason: {:?}", err);
    process::exit(1);
}

fn search_directories() -> Vec<PathBuf> {
    let home = env::home_dir().unwrap();
    let mut dirs = vec![home.join("bin")];
    if let Some(val) = env::var_os("PATH") {
        dirs.extend(env::split_paths(&val));
    }
    dirs
}

#[cfg(unix)]
fn is_executable<P: AsRef<Path>>(path: P) -> bool {
    use std::os::unix::prelude::*;
    fs::metadata(path)
        .map(|metadata| metadata.is_file() && metadata.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}
#[cfg(windows)]
fn is_executable<P: AsRef<Path>>(path: P) -> bool {
    fs::metadata(path).map(|metadata| metadata.is_file()).unwrap_or(false)
}
