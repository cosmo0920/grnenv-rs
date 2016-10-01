use std::env;
use std::fs;

use config::Config;

pub fn init() {
    let config = Config::new();
    if config.install_dir.exists() || config.shim_dir.exists() {
        println!("Already initalized. Reinitializing....");
    }
    fs::create_dir_all(&config.install_dir).expect("Could not create installation directory.");
    fs::create_dir_all(&config.shim_dir).expect("Could not create shims directory.");
    fs::create_dir_all(&config.versions_dir).expect("Could not create shims directory.");
    if !env::home_dir()
        .unwrap()
        .join(".profile")
        .exists() {
        println!(r#"Write the following thing:

. $HOME\.groonga\shims\bin\source-groonga.sh
"#)
    }
}
