use std::env;
use std::fs;
use std::io::prelude::*;
use std::io;
use std::process;
use toml;
use rustc_serialize::Decodable;
use config::Config;

const DEFAULT_ARGS: &'static str = "\"--with-zlib --with-ssl --enable-mruby --without-libstemmer \
                                    --disable-zeromq\"";

#[derive(Debug, PartialEq, Eq, RustcDecodable, RustcEncodable)]
pub struct Configuration {
    settings: BuildConfig,
}

#[derive(Debug, Clone, PartialEq, Eq, RustcDecodable, RustcEncodable)]
pub struct BuildConfig {
    args: String,
}

pub fn write_conf(config: &Config) {
    if !config.build_conf.exists() {
        let mut f = fs::File::create(&config.build_conf)
            .expect("Cound not create build configuration file.");
        let contents = format!("[settings]\nargs = {}", DEFAULT_ARGS);
        match f.write_all(&contents.as_bytes()) {
            Ok(_) => (),
            Err(e) => {
                println!("{}", e);
                process::exit(2)
            }
        }
    }
}

pub fn parse_toml(config_content: String) -> BuildConfig {
    println!("config:\n{}", config_content);
    let mut parser = toml::Parser::new(&*config_content);
    let toml = match parser.parse() {
        Some(toml) => toml::Value::Table(toml),
        None => panic!("Couldn't parse toml"),
    };
    let mut decoder = toml::Decoder::new(toml);
    let config = match Configuration::decode(&mut decoder) {
        Ok(config) => config,
        Err(_) => panic!("Couldn't decode toml with Configuration struct"),
    };
    config.settings
}

pub fn read_build_args(config: &Config) -> Result<String, io::Error> {
    let mut f = fs::File::open(&config.build_conf)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    let build_conf = parse_toml(buffer);
    Ok(build_conf.args)
}

// mainly used for macOS.
fn openssl_pkg_config_path() -> String {
    let key = "OPENSSL_PKG_CONFIG_PATH";
    env::var(key).unwrap_or("/usr/local/opt/openssl/lib/pkgconfig".to_string())
}

pub fn build_args(config: &Config, groonga_dir: String) -> Result<Vec<String>, io::Error> {
    let conf_args = read_build_args(config)?;
    let build_args: Vec<String> =
        conf_args.split_whitespace().into_iter().map(|e| e.to_owned()).collect();
    println!("{} with args: {:?}",
             config.versions_dir.join(groonga_dir.clone()).display(),
             build_args.clone());
    let mut args = vec![format!("--prefix={}",
                                config.versions_dir.join(groonga_dir.clone()).display()),
                        format!("PKG_CONFIG_PATH={}", openssl_pkg_config_path())];
    args.extend(build_args.iter().cloned());
    Ok(args)
}

#[cfg(not(target_os = "windows"))]
pub fn is_program_in_path(program: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        for p in path.split(":") {
            let p_str = format!("{}/{}", p, program);
            if fs::metadata(p_str).is_ok() {
                return true;
            }
        }
    }
    false
}

#[cfg(target_os = "windows")]
pub fn is_program_in_path(program: &str) -> bool {
    use std::process::{Command, Stdio};

    let mut args = vec!["-Command".to_string(), "Get-Command".to_string()];
    args.push(program.to_string());
    Command::new("powershell")
        .args(&*args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .is_ok()
}

pub fn make() -> Option<&'static str> {
    if is_program_in_path("gmake") {
        Some("gmake")
    } else if is_program_in_path("make") {
        Some("make")
    } else {
        None
    }
}

pub fn cmake() -> Option<&'static str> {
    if is_program_in_path("cmake") {
        Some("cmake")
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[cfg(target_os = "windows")]
    fn test_is_program_in_path() {
        let cmd = is_program_in_path("cmd");
        assert_eq!(true, cmd);
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_make() {
        let cmake = make();
        assert_eq!(Some("make"), cmake);
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_make() {
        let cmake = make();
        assert_eq!(Some("make"), cmake);
    }

    #[test]
    fn test_parse_toml() {
        let toml = r#"[settings]
args = "an example arguments"
"#;
        let settings = parse_toml(toml.to_string());
        let expected = BuildConfig { args: "an example arguments".to_string() };
        assert_eq!(expected, settings);
    }
}
