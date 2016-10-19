use std::fs;
use std::io::prelude::*;
use std::io;
use std::process;
use toml;
use rustc_serialize::Decodable;
use config::Config;

const DEFAULT_ARGS: &'static str = "--with-zlib --with-ssl --enable-mruby\
                                    --without-libstemmer --disable-zeromq";

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct Configuration {
    settings: BuildConfig,
}

#[derive(Debug, Clone, PartialEq, Eq, RustcDecodable, RustcEncodable)]
struct BuildConfig {
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
            },
        }
    }
}

fn parse_toml(config_content: String) -> BuildConfig {
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
    let mut f = try!(fs::File::open(&config.build_conf));
    let mut buffer = String::new();
    try!(f.read_to_string(&mut buffer));
    let build_conf = parse_toml(buffer);
    Ok(build_conf.args)
}
