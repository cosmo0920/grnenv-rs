extern crate clap;

use std::env;
use std::fs;

use clap::ArgMatches;
use tempdir::TempDir;
use hyper::Client;
use util;
use extractor;
use downloader;
use profile;

pub fn init() {
    let install_dir = util::obtain_install_base_path();
    let shim_dir = install_dir.join("shims").join("bin");
    if install_dir.exists() || shim_dir.exists() {
        println!("Already initalized. Reinitializing....");
    }
    fs::create_dir_all(&install_dir).expect("Could not create installation directory.");
    fs::create_dir_all(&shim_dir).expect("Could not create shims directory.");
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
    let default_arch = util::obtain_arch().expect("Unsupported architecture.");
    let arch = match m.value_of("arch").unwrap_or(&*default_arch) {
        "x64" | "x86_64" => "x64",
        "x86" | "i686" => "x86",
        _ => panic!("Invalid architecture specified."),
    };
    let version = m.value_of("VERSION").unwrap();
    println!("Value for architecture: {}", arch);
    println!("Obtaining Groonga version: {}", version);
    let install_base_dir = util::obtain_install_base_path();
    let shim_dir = install_base_dir.join("shims").join("bin");

    let groonga_versioned_dir = util::obtain_groonga_versioned_path();
    let groonga_dir = format!("groonga-{}-{}", version, arch);
    let groonga_binary = format!("{}.zip", groonga_dir.clone());
    if groonga_versioned_dir.join(groonga_dir.clone()).exists() {
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
    extractor::extract_zip(&filename, &groonga_versioned_dir);
    profile::create_profile_source(&shim_dir, &groonga_dir, &groonga_versioned_dir)
        .expect("Could not create source-groonga.ps1");
}

pub fn switch(m: &ArgMatches) {
    let default_arch = util::obtain_arch().expect("Unsupported architecture.");
    let arch = match m.value_of("arch").unwrap_or(&*default_arch) {
        "x64" | "x86_64" => "x64",
        "x86" | "i686" => "x86",
        _ => panic!("Invalid architecture specified."),
    };
    let version = m.value_of("VERSION").unwrap();
    println!("Value for architecture: {}", arch);
    println!("Using Groonga version: {}", version);
    let install_dir = util::obtain_install_base_path();
    let groonga_versioned_dir = util::obtain_groonga_versioned_path();
    let shim_dir = install_dir.join("shims").join("bin");
    let groonga_dir = format!("groonga-{}-{}", version, arch);

    profile::create_profile_source(&shim_dir, &groonga_dir, &groonga_versioned_dir)
        .expect("Could not create source-groonga.ps1");
}
