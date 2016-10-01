extern crate tempdir;
extern crate hyper;
extern crate grnenvlib;
extern crate clap;

use clap::{Arg, App, SubCommand};
use grnenvlib::command;

fn main() {
    default_main();
}

fn cli() -> App<'static, 'static> {
    App::new("grnenv-rs")
        .version("0.1.0")
        .author("Hiroshi Hatake <cosmo0920.wp@gmail.com>")
        .about("A tiny tool for obtain and select multiple Groonga.")
        .subcommand(SubCommand::with_name("init").about("Prepare grnenv-rs."))
        .subcommand(SubCommand::with_name("install")
            .about("Install a given Groonga version and arch")
            .arg(Arg::with_name("VERSION").required(true))
            .arg(Arg::with_name("arch")
                .short("a")
                .long("arch")
                .value_name("ARCH")
                .help("Select architectures. e.g.) x86, x64")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("switch")
            .about("switch Groonga with given version and arch")
            .arg(Arg::with_name("VERSION").required(true))
            .arg(Arg::with_name("arch")
                .short("a")
                .long("arch")
                .value_name("ARCH")
                .help("Select architectures. e.g.) x86, x64")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("versions").about("display installed Groonga versions"))
}

#[cfg(not(windows))]
fn default_main() {
    panic!("Unsupported!");
}

#[cfg(windows)]
fn default_main() {
    let matches = cli().get_matches();
    match matches.subcommand() {
        ("init", _) => command::windows::init(),
        ("install", Some(m)) => command::windows::install(m),
        ("switch", Some(m)) => command::windows::switch(m),
        ("versions", _) => command::windows::versions(),
        (_, _) => unreachable!(),
    }
}
