use std::env;
use std::fs;
use std::path::Path;

use clap::ArgMatches;
use tempdir::TempDir;
use hyper::Client;
use util;
use extractor;
use downloader;
use profile;
use config::Config;

pub fn init() {
    let config = Config::new();
    if config.install_dir.exists() || config.shim_dir.exists() {
        println!("Already initalized. Reinitializing....");
    }
    fs::create_dir_all(&config.install_dir).expect("Could not create installation directory.");
    fs::create_dir_all(&config.shim_dir).expect("Could not create shims directory.");
    if !env::home_dir()
        .unwrap()
        .join("Documents")
        .join("WindowsPowerShell")
        .join("profile.ps1")
        .exists() {
        println!(r#"Please create profile.ps1 the following place:

$Env:USERPROFILE\Documents\WindowsPowerShell\profile.ps1

And write the following thing:

. $Env:USERPROFILE\.groonga\shims\bin\source-groonga.ps1
"#)
    }
}

pub fn install(m: &ArgMatches) {
    const BASE_URL: &'static str = "http://packages.groonga.org/windows/groonga";
    let config = Config::from_matches(m);
    println!("Value for architecture: {}",
             config.arch.clone().expect("unsupported platform"));
    println!("Obtaining Groonga version: {}",
             config.version.clone().unwrap());
    let groonga_dir = format!("groonga-{}-{}",
                              config.version.unwrap(),
                              config.arch.unwrap());
    let groonga_binary = format!("{}.zip", groonga_dir.clone());
    if config.versions_dir.join(groonga_dir.clone()).exists() {
        println!("Already installed. Ready to use it.");
        return ();
    }

    let download_dir = TempDir::new("grnenv-rs")
        .expect("Could not create temp dir.")
        .into_path();

    let client = Client::new();
    let filename = downloader::file_download(&client,
                                             &*format!("{}/{}", BASE_URL, groonga_binary),
                                             download_dir)
        .expect("Failed to download");
    extractor::extract_zip(&filename, &config.versions_dir);
}

pub fn switch(m: &ArgMatches) {
    let config = Config::from_matches(m);
    println!("Value for architecture: {}",
             config.arch.clone().expect("unsupported platform"));
    println!("Using Groonga version: {}", config.version.clone().unwrap());
    let groonga_dir = format!("groonga-{}-{}",
                              config.version.unwrap(),
                              config.arch.unwrap());
    match config.version {
        Some("system") => {
            profile::remove_grnenv_profile(&config.shim_dir).unwrap();
            return ();
        }
        Some(_) => {
            profile::create_profile_source(&config.shim_dir, &groonga_dir, &config.versions_dir)
                .expect("Could not create source-groonga.ps1")
        }
        None => unreachable!(),
    }
}

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
        println!("\t{}", entry);
    }
}
