use std::process;
use std::process::{Command, Stdio};

fn main() {
    #[cfg(target_os = "macos")]
    fn inner() -> &'static str {
        "open"
    }
    #[cfg(target_os = "windows")]
    fn inner() -> &'static str {
        "start"
    }
    #[cfg(target_os = "linux")]
    fn inner() -> &'static str {
        "xdg-open"
    }
    #[cfg(target_os = "unix")]
    fn inner() -> &'static str {
        "xdg-open"
    }

    let command = inner();
    let arg = "http://groonga.org/docs/";
    let err = match Command::new(command)
        .args(&[arg])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn() {
        Ok(_) => return (),
        Err(e) => e,
    };
    println!("Failed to execute docopen subcommand. reason: {:?}", err);
    process::exit(1);

}
