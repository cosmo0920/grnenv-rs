use std::fs;
use std::io::prelude::*;
use std::io;
use std::path::PathBuf;
use std::process;

pub fn create_profile_source(shim_dir: &PathBuf,
                             groonga_dir: &String,
                             install_dir: &PathBuf)
                             -> Result<(), io::Error> {
    if shim_dir.join("source-groonga.ps1").exists() {
        let _ = fs::remove_file(shim_dir.join("source-groonga.ps1"))?;
    }
    let profile = shim_dir.join("source-groonga.ps1");
    let path = profile.to_str()
        .to_owned()
        .ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData,
                           "Could not convert str from PathBuf.")
        })?;
    let mut f = fs::File::create(path).expect("Could not create a powershell setting file.");
    let installed_groonga = install_dir.join(groonga_dir)
        .join("bin");
    if !installed_groonga.exists() {
        println!("Specified version is not installed.");
        process::exit(1);
    }
    let contents = format!("$Env:Path = \"{};\" + $Env:Path",
                           installed_groonga.display());
    match f.write_all(&contents.as_bytes()) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
    f.sync_data()
}

pub fn remove_grnenv_profile(shim_dir: &PathBuf) -> Result<(), io::Error> {
    if shim_dir.join("source-groonga.ps1").exists() {
        let _ = fs::remove_file(shim_dir.join("source-groonga.ps1"))?;
    }
    // Create an empty file to prevent reading source-groonga.ps1 error.
    let source_dir = shim_dir.join("source-groonga.ps1");
    let path = source_dir.to_str()
        .to_owned()
        .ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData,
                           "Could not convert str from PathBuf.")
        })?;
    let _ = fs::File::create(path)?;
    Ok(())
}
