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
    pub build_conf: PathBuf,
}

impl<'a> Default for Config<'a> {
    fn default() -> Config<'a> {
        Config {
            install_dir: PathBuf::from(""),
            shim_dir: PathBuf::from(""),
            versions_dir: PathBuf::from(""),
            arch: None,
            version: None,
            build_conf: PathBuf::from(""),
        }
    }
}

impl<'a> Config<'a> {
    pub fn new() -> Config<'a> {
        let install_dir = util::obtain_install_base_path();
        let shim_dir = install_dir.join("shims").join("bin");
        let groonga_versioned_dir = util::obtain_groonga_versioned_path();
        let build_conf = install_dir.join("build.toml");
        Config {
            install_dir: install_dir,
            shim_dir: shim_dir,
            versions_dir: groonga_versioned_dir,
            build_conf: build_conf,
            ..Config::default()
        }
    }

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
        let build_conf = install_dir.join("build.toml");
        Config {
            install_dir: install_dir,
            shim_dir: shim_dir,
            versions_dir: groonga_versioned_dir,
            arch: arch,
            version: version,
            build_conf: build_conf,
        }
    }
}
