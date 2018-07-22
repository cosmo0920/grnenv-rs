use std::borrow::Cow;
use std::env;
use std::fs;
use std::io;
use std::process;

use clap::ArgMatches;
use kuchiki;
use tempdir::TempDir;

use config::Config;
use downloader;
use extractor;
use profile;
use reqwest::{Client, Proxy, Url};

pub fn init() {
    let config = Config::new();
    if config.install_dir.exists() || config.shim_dir.exists() {
        println!("Already initalized. Reinitializing....");
    }
    fs::create_dir_all(&config.install_dir).expect("Could not create installation directory.");
    fs::create_dir_all(&config.shim_dir).expect("Could not create shims directory.");
    fs::create_dir_all(&config.versions_dir).expect("Could not create versions directory.");
    if !env::home_dir()
        .unwrap_or_else(|| panic!("Cound not found homedir."))
        .join("Documents")
        .join("WindowsPowerShell")
        .join("profile.ps1")
        .exists()
    {
        println!(
            r#"Please create profile.ps1 in the following place:

$Env:USERPROFILE\Documents\WindowsPowerShell\profile.ps1

And write the following thing:

. $Env:USERPROFILE\.groonga\shims\bin\source-groonga.ps1
"#
        )
    }
}

pub fn install(m: &ArgMatches) {
    extern crate env_proxy;

    const BASE_URL: &'static str = "http://packages.groonga.org/windows/groonga";
    let maybe_proxy = env_proxy::for_url(&Url::parse(BASE_URL).unwrap());
    let config = Config::from_matches(m);
    println!(
        "Value for architecture: {}",
        config.arch.clone().expect("unsupported platform")
    );
    println!(
        "Obtaining Groonga version: {}",
        config.version.clone().unwrap()
    );
    let groonga_dir = format!(
        "groonga-{}-{}",
        config.version.unwrap(),
        config.arch.unwrap()
    );
    let groonga_binary = format!("{}.zip", groonga_dir.clone());
    if config.versions_dir.join(groonga_dir.clone()).exists() {
        println!("Already installed. Ready to use it.");
        return ();
    }

    let download_dir = TempDir::new("grnenv-rs")
        .expect("Could not create temp dir.")
        .into_path();

    let client = match maybe_proxy {
        None => Client::new(),
        Some(_) => Client::builder()
            .proxy(Proxy::http(BASE_URL).unwrap())
            .build()
            .expect("Could not create proxy."),
    };
    let filename = downloader::file_download(
        &client,
        &*format!("{}/{}", BASE_URL, groonga_binary),
        download_dir,
        "groonga.zip",
    ).expect("Failed to download");
    assert!(extractor::extract_zip(&filename, &config.versions_dir).is_ok());
}

pub fn switch(m: &ArgMatches) {
    let config = Config::from_matches(m);
    println!(
        "Value for architecture: {}",
        config.arch.clone().expect("unsupported platform")
    );
    println!("Using Groonga version: {}", config.version.clone().unwrap());
    let groonga_dir = format!(
        "groonga-{}-{}",
        config.version.unwrap(),
        config.arch.unwrap()
    );
    match config.version {
        Some("system") => {
            profile::windows::remove_grnenv_profile(&config.shim_dir).unwrap();
            return ();
        }
        Some(_) => profile::windows::create_profile_source(
            &config.shim_dir,
            &groonga_dir,
            &config.versions_dir,
        ).expect("Could not create source-groonga.ps1"),
        None => unreachable!(),
    }
}

pub fn uninstall(m: &ArgMatches) {
    let config = Config::from_matches(m);
    let mut choice = String::new();
    let arch = match config.arch.unwrap() {
        Cow::Borrowed(s) => s.to_owned(),
        Cow::Owned(s) => s,
    };
    let groonga_dir = format!("groonga-{}-{}", config.version.unwrap(), arch);
    if config.versions_dir.join(groonga_dir.clone()).exists() {
        println!(
            "Uninstall Groonga version {}, arch {}? [y/N]",
            config.version.unwrap(),
            arch
        );
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        if choice == "y".to_owned() || choice == "Y".to_owned() {
            println!("Removing {}....", groonga_dir.clone());
            fs::remove_dir_all(&config.versions_dir.join(groonga_dir))
                .expect("Could not remove specified directory.");
        }
    } else {
        println!("{} is not installed!", groonga_dir.clone());
        process::exit(1);
    }
}

pub fn list() {
    extern crate env_proxy;
    use kuchiki::traits::*;

    let base_url: &'static str = "http://packages.groonga.org/windows/groonga";
    let maybe_proxy = env_proxy::for_url(&Url::parse(base_url).unwrap());
    let client = match maybe_proxy {
        None => Client::new(),
        Some(_) => Client::builder()
            .proxy(Proxy::http(base_url).unwrap())
            .build()
            .expect("Could not create proxy."),
    };
    let page = downloader::page_download(&client, &*format!("{}", base_url))
        .expect("Failed to download page");
    let doc = kuchiki::parse_html().one(page);
    let docs = doc.select("tr").unwrap().collect::<Vec<_>>();
    println!("Installable Groonga:");
    for handle in &docs {
        let texts = handle
            .as_node()
            .descendants()
            .text_nodes()
            .collect::<Vec<_>>();
        if let Some(text) = texts.first() {
            let package = text.as_node().text_contents();
            if package.contains("groonga")
                && package.contains("zip")
                && (package.contains("x86") || package.contains("x64"))
            {
                let package = package.split(".zip").collect::<Vec<_>>();
                let pkg = package
                    .first()
                    .unwrap_or(&"")
                    .to_owned()
                    .split("-")
                    .collect::<Vec<_>>();
                println!(
                    "\t{} --arch {}",
                    pkg.get(1).unwrap_or(&""),
                    pkg.get(2).unwrap_or(&"")
                );
            }
        }
    }
}
