use std::process;
use std::process::{Command, Stdio};

fn main() {
    #[cfg(target_os = "macos")]
    fn inner() -> &'static str {
        "open"
    }
    #[cfg(target_os = "windows")]
    fn inner() -> &'static str {
        "powershell"
    }
    #[cfg(target_os = "linux")]
    fn inner() -> &'static str {
        "xdg-open"
    }
    #[cfg(target_os = "unix")]
    fn inner() -> &'static str {
        "xdg-open"
    }

    #[cfg(not(target_os = "windows"))]
    fn arg() -> [&'static str; 1] {
        ["http://groonga.org/docs/"]
    }
    #[cfg(target_os = "windows")]
    fn arg() -> [&'static str; 3] {
        ["-Command", "Start-Process", "http://groonga.org/docs"]
    }

    let command = inner();
    let arg = arg();
    match Command::new(command)
        .args(&arg)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn() {
        Ok(_) => return (),
        Err(e) => {
            println!("Failed to execute docopen subcommand. reason: {:?}", e);
            process::exit(1);
        }
    };
}
