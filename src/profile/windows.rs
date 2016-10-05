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
        let _ = try!(fs::remove_file(shim_dir.join("source-groonga.ps1")));
    }
    let mut f = fs::File::create(shim_dir.join("source-groonga.ps1").to_str().unwrap())
        .expect("Could not create a powershell setting file.");
    let installed_groonga = install_dir.join(groonga_dir)
        .join("bin");
    if !installed_groonga.exists() {
        println!("Specified version is not installed.");
        process::exit(1);
    }
    let contents = format!("$Env:Path = \"{};\" + $Env:Path",
                           installed_groonga.display());
    match f.write_all(&contents.as_bytes()) {
        Ok(_) => return Ok(()),
        Err(e) => println!("{}", e),
    }
    f.sync_data()
}

pub fn remove_grnenv_profile(shim_dir: &PathBuf) -> Result<(), io::Error> {
    if shim_dir.join("source-groonga.ps1").exists() {
        let _ = try!(fs::remove_file(shim_dir.join("source-groonga.ps1")));
    }
    // Create an empty file to prevent reading source-groonga.ps1 error.
    let _ = try!(fs::File::create(shim_dir.join("source-groonga.ps1").to_str().unwrap()));
    Ok(())
}
