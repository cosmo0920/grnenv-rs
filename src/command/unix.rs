use std::env;
use std::fs;
use std::io;
use std::process;
use std::process::{Command, Stdio};
use sys_info;
use kuchiki;

use clap::ArgMatches;
use tempdir::TempDir;
use hyper::{Client, Url};
use config::Config;
use extractor;
use downloader;
use profile;

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
        .join(".profile")
        .exists() {
        println!(r#"Write the following thing:

. $HOME\.groonga\shims\bin\source-groonga.sh
"#)
    }
}

pub fn install(m: &ArgMatches) {
    extern crate env_proxy;

    const BASE_URL: &'static str = "http://packages.groonga.org/source/groonga";
    let maybe_proxy = env_proxy::for_url(&Url::parse(BASE_URL).unwrap());
    let config = Config::from_matches(m);
    println!("Obtaining Groonga version: {}",
             config.version.clone().unwrap());
    let groonga_dir = format!("groonga-{}", config.version.unwrap());
    let groonga_source = format!("{}.zip", groonga_dir.clone());
    if config.versions_dir.join(groonga_dir.clone()).exists() {
        println!("Already installed. Ready to use it.");
        return ();
    }

    let download_dir = TempDir::new("grnenv-rs")
        .expect("Could not create temp dir.")
        .into_path();

    let client = match maybe_proxy {
        None => Client::new(),
        Some(host_port) => Client::with_http_proxy(host_port.0, host_port.1)
    };
    let filename = downloader::file_download(&client,
                                             &*format!("{}/{}", BASE_URL, groonga_source),
                                             download_dir.clone(),
                                             "groonga.zip")
        .expect("Failed to download");
    extractor::extract_zip(&filename, &download_dir);
    assert!(env::set_current_dir(&download_dir.join(groonga_dir.clone())).is_ok());

    // TODO: Should specify on Linux?
    match inner_autoreconf() {
        Ok(o) => println!("{}", o),
        Err(_) => {
            println!("Could not execute autoreconf -v.");
            process::exit(1);
        }
    }

    match inner_configure(&config, groonga_dir.clone()) {
        Ok(o) => println!("{}", o),
        Err(_) => {
            println!("Could not configure.");
            process::exit(1);
        }
    }

    match inner_build() {
        Ok(o) => println!("{}", o),
        Err(_) => {
            println!("Could not build.");
            process::exit(1);
        }
    }

    match inner_install() {
        Ok(o) => println!("{}", o),
        Err(_) => {
            println!("Could not install.");
            process::exit(1);
        }
    }

    #[cfg(target_os = "macos")]
    fn inner_autoreconf() -> Result<process::ExitStatus, io::Error> {
        let mut cmd = Command::new("autoreconf")
            .args(&["-v"])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap_or_else(|e| panic!("failed to execute child: {}", e));
        let status = cmd.wait();
        status
    }

    #[cfg(not(target_os = "macos"))]
    fn inner_autoreconf() -> Result<u32, io::Error> {
        Ok(0)
    }

    // mainly used for macOS.
    fn openssl_pkg_config_path() -> String {
        let key = "OPENSSL_PKG_CONFIG_PATH";
        env::var(key).unwrap_or("/usr/local/opt/openssl/lib/pkgconfig".to_string())
    }

    fn inner_configure(config: &Config,
                       groonga_dir: String)
                       -> Result<process::ExitStatus, io::Error> {
        println!("{}",
                 config.versions_dir.join(groonga_dir.clone()).display());
        let mut cmd = Command::new("./configure")
            .args(&[&*format!("--prefix={}",
                              config.versions_dir.join(groonga_dir.clone()).display()),
                    &*format!("PKG_CONFIG_PATH={}", openssl_pkg_config_path())])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap_or_else(|e| panic!("failed to execute child: {}", e));
        let status = cmd.wait();
        status
    }

    fn inner_build() -> Result<process::ExitStatus, io::Error> {
        let mut cmd = Command::new("make")
            .args(&["-j", &*format!("{}", sys_info::cpu_num().unwrap_or(2))])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap_or_else(|e| panic!("failed to execute child: {}", e));
        let status = cmd.wait();
        status
    }

    fn inner_install() -> Result<process::ExitStatus, io::Error> {
        let mut cmd = Command::new("make")
            .args(&["install"])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap_or_else(|e| panic!("failed to execute child: {}", e));
        let status = cmd.wait();
        status
    }
}

pub fn switch(m: &ArgMatches) {
    let config = Config::from_matches(m);
    println!("Using Groonga version: {}", config.version.clone().unwrap());
    let groonga_dir = format!("groonga-{}", config.version.unwrap());
    match config.version {
        Some("system") => {
            profile::unix::remove_grnenv_profile(&config.shim_dir).unwrap();
            return ();
        }
        Some(_) => {
            profile::unix::create_profile_source(&config.shim_dir,
                                                 &groonga_dir,
                                                 &config.versions_dir)
                .expect("Could not create source-groonga.sh")
        }
        None => unreachable!(),
    }
}

pub fn uninstall(m: &ArgMatches) {
    let config = Config::from_matches(m);
    let groonga_dir = format!("groonga-{}", config.version.unwrap());
    if config.versions_dir.join(groonga_dir.clone()).exists() {
        println!("Removing {}....", groonga_dir.clone());
        fs::remove_dir_all(&config.versions_dir.join(groonga_dir))
            .expect("Could not remove specified directory.");
    } else {
        println!("{} is not installed!", groonga_dir.clone());
        process::exit(1);
    }
}


pub fn list() {
    use kuchiki::traits::*;

    if let Ok(doc) = kuchiki::parse_html().from_http("http://packages.groonga.org/source/groonga") {
        let docs = doc.select("tr")
            .unwrap_or_else(|e| panic!("failed to find tr elements: {:?}", e))
            .collect::<Vec<_>>();
        println!("Installable Groonga:");
        for handle in &docs {
            let texts = handle.as_node().descendants().text_nodes().collect::<Vec<_>>();
            if let Some(text) = texts.first() {
                let package = text.as_node().text_contents();
                if package.contains("groonga") && package.contains("zip") &&
                   !package.contains("asc") {
                    let display = package.split(".zip").collect::<Vec<_>>();
                    println!("\t{}", display.first().unwrap_or(&""));
                }
            }
        }
    } else {
        println!("{}", "The page couldn't be fetched");
    }
}
