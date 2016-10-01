use std::path::PathBuf;
use std::borrow::Cow;
use clap::ArgMatches;
use util;

#[derive(Debug, Clone)]
pub struct Config<'a> {
    pub install_dir: PathBuf,
    pub shim_dir: PathBuf,
    pub versions_dir: PathBuf,
    pub arch: Option<Cow<'a, str>>,
    pub version: Option<&'a str>,
}

impl<'a> Config<'a> {
    pub fn from_matches(m: &'a ArgMatches) -> Config<'a> {
        let install_dir = util::obtain_install_base_path();
        let shim_dir = install_dir.join("shims").join("bin");
        let groonga_versioned_dir = util::obtain_groonga_versioned_path();
        let version = m.value_of("VERSION");
        let default_arch = util::obtain_arch().expect("Unsupported architecture.");
        let arch = match m.value_of("arch").unwrap_or(&*default_arch) {
            "x64" | "x86_64" => Some(Cow::Borrowed("x64")),
            "x86" | "i686" => Some(Cow::Borrowed("x86")),
            _ => None,
        };
        Config {
            install_dir: install_dir,
            shim_dir: shim_dir,
            versions_dir: groonga_versioned_dir,
            arch: arch,
            version: version,
        }
    }
}
