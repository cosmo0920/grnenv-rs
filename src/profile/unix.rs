use std::fs;
use std::io::prelude::*;
use std::io;
use std::path::PathBuf;
use std::process;

pub fn create_profile_source(shim_dir: &PathBuf,
                             groonga_dir: &String,
                             install_dir: &PathBuf)
                             -> Result<(), io::Error> {
    #[cfg(not(target_os = "macos"))]
    fn dynamic_link_var_pathname() -> String {
        "LD_LIBRARY_PATH".to_string()
    }
    #[cfg(target_os = "macos")]
    fn dynamic_link_var_pathname() -> String {
        "DYLD_LIBRARY_PATH".to_string()
    }
    if shim_dir.join("source-groonga.sh").exists() {
        let _ = fs::remove_file(shim_dir.join("source-groonga.sh"))?;
    }
    let mut f = fs::File::create(shim_dir.join("source-groonga.sh").to_str().unwrap())
        .expect("Could not create a shell setting file.");
    let installed_groonga = install_dir.join(groonga_dir)
        .join("bin");
    let libgroonga = install_dir.join(groonga_dir)
        .join("lib");
    let pkg_config = install_dir.join(groonga_dir)
        .join("lib")
        .join("pkgconfig");
    if !installed_groonga.exists() {
        println!("Specified version is not installed.");
        process::exit(1);
    }
    let contents = format!("export PATH={}:$PATH\nexport {}={}:${}\nexport \
                            PKG_CONFIG_PATH={}:$PKG_CONFIG_PATH",
                           installed_groonga.display(),
                           dynamic_link_var_pathname(),
                           libgroonga.display(),
                           dynamic_link_var_pathname(),
                           pkg_config.display());
    match f.write_all(&contents.as_bytes()) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
    f.sync_data()
}

pub fn remove_grnenv_profile(shim_dir: &PathBuf) -> Result<(), io::Error> {
    if shim_dir.join("source-groonga.sh").exists() {
        let _ = fs::remove_file(shim_dir.join("source-groonga.sh"))?;
    }
    // Create an empty file to prevent reading source-groonga.sh error.
    let _ = fs::File::create(shim_dir.join("source-groonga.sh").to_str().unwrap())?;
    Ok(())
}
