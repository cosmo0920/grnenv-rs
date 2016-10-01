extern crate tempdir;
extern crate hyper;
extern crate grnenvlib;
extern crate clap;

use std::fs;
use std::io::prelude::*;

use clap::{Arg, App, SubCommand, ArgMatches};
use tempdir::TempDir;
use hyper::Client;
use grnenvlib::util;
use grnenvlib::extractor;
use grnenvlib::downloader;
use grnenvlib::profile;

fn main() {
    default_main();
}

fn cli() -> App<'static, 'static> {
    App::new("grnenv-rs")
        .version("0.1.0")
        .author("Hiroshi Hatake <cosmo0920.wp@gmail.com>")
        .about("A tiny tool for obtain and select multiple Groonga.")
        .subcommand(SubCommand::with_name("init").about("Prepare grnenv-rs."))
        .subcommand(SubCommand::with_name("install")
            .about("Install a given Groonga version and arch")
            .arg(Arg::with_name("VERSION").required(true))
            .arg(Arg::with_name("arch")
                .short("a")
                .long("arch")
                .value_name("ARCH")
                .help("Select architectures. e.g.) x86, x64")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("switch")
            .about("switch Groonga with given version and arch")
            .arg(Arg::with_name("VERSION").required(true))
            .arg(Arg::with_name("arch")
                .short("a")
                .long("arch")
                .value_name("ARCH")
                .help("Select architectures. e.g.) x86, x64")
                .takes_value(true)))
}

#[cfg(not(windows))]
fn default_main() {
    panic!("Unsupported!");
}

fn init() {
    let install_dir = util::obtain_install_path();
    let shim_dir = install_dir.join("shims").join("bin");
    if install_dir.exists() || shim_dir.exists() {
        println!("Already initalized. Reinitializing....");
    }
    fs::create_dir_all(&install_dir).expect("Could not create installation directory.");
    fs::create_dir_all(&shim_dir).expect("Could not create shims directory.");
    if !std::env::home_dir()
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

fn install(m: &ArgMatches) {
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
    let install_dir = util::obtain_install_path();
    let shim_dir = install_dir.join("shims").join("bin");

    let groonga_dir = format!("groonga-{}-{}", version, arch);
    let groonga_binary = format!("{}.zip", groonga_dir.clone());
    if install_dir.join(groonga_dir.clone()).exists() {
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
    extractor::extract_zip(&filename, &install_dir);
    profile::create_profile_source(&shim_dir, &groonga_dir, &install_dir);
}

fn switch(m: &ArgMatches) {
    let default_arch = util::obtain_arch().expect("Unsupported architecture.");
    let arch = match m.value_of("arch").unwrap_or(&*default_arch) {
        "x64" | "x86_64" => "x64",
        "x86" | "i686" => "x86",
        _ => panic!("Invalid architecture specified."),
    };
    let version = m.value_of("VERSION").unwrap();
    println!("Value for architecture: {}", arch);
    println!("Obtaining Groonga version: {}", version);
    let install_dir = util::obtain_install_path();
    let shim_dir = install_dir.join("shims").join("bin");
    let groonga_dir = format!("groonga-{}-{}", version, arch);

    profile::create_profile_source(&shim_dir, &groonga_dir, &install_dir);
}

#[cfg(windows)]
fn default_main() {
    let matches = cli().get_matches();
    match matches.subcommand() {
        ("init", _) => init(),
        ("install", Some(m)) => install(m),
        ("switch", Some(m)) => switch(m),
        (_, _) => unreachable!(),
    }
}
