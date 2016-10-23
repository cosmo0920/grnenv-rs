use std::env;
use std::path::PathBuf;
#[cfg(windows)]
use kernel32::GetNativeSystemInfo;

pub fn obtain_groonga_versioned_path() -> PathBuf {
    let base_dir = env::home_dir().unwrap();
    base_dir.join(".groonga").join("versions")
}

pub fn obtain_install_base_path() -> PathBuf {
    let base_dir = env::home_dir().unwrap();
    base_dir.join(".groonga")
}

#[cfg(windows)]
pub fn obtain_arch() -> Option<String> {
    use std::mem;

    // Detect architectures
    const PROCESSOR_ARCHITECTURE_AMD64: u16 = 9;
    const PROCESSOR_ARCHITECTURE_INTEL: u16 = 0;
    let mut sys_info;
    unsafe {
        sys_info = mem::zeroed();
        GetNativeSystemInfo(&mut sys_info);
    }

    let arch = match sys_info.wProcessorArchitecture {
        PROCESSOR_ARCHITECTURE_AMD64 => "x64",
        PROCESSOR_ARCHITECTURE_INTEL => "x86",
        _ => return None,
    };
    Some(arch.to_string())
}

#[cfg(not(windows))]
pub fn obtain_arch() -> Option<String> {
    use libc;
    use std::mem;
    use std::ffi::CStr;

    let mut sys_info;
    let machine = unsafe {
        sys_info = mem::zeroed();
        if libc::uname(&mut sys_info) != 0 {
            return None;
        }

        CStr::from_ptr(sys_info.machine.as_ptr()).to_bytes()
    };
    Some(String::from_utf8_lossy(machine).into_owned())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::env;
    use std::path::PathBuf;

    #[cfg(target_os = "linux")]
    fn stub_home() -> &'static str {
        "/home/grnenv"
    }
    #[cfg(target_os = "macos")]
    fn stub_home() -> &'static str {
        "/Users/grnenv"
    }
    #[cfg(target_os = "windows")]
    fn stub_home() -> &'static str {
        "C:\\Users\\grnenv"
    }

    #[test]
    fn test_obtain_arch() {
        let arch = obtain_arch();
        assert!(arch.is_some());
    }

    #[test]
    fn test_obtain_groonga_versioned_path() {
        let home = env::home_dir().unwrap();
        let stub_home = stub_home();
        let _ = env::set_var("HOME", stub_home.clone());
        let path = obtain_groonga_versioned_path();
        let versioned = PathBuf::from(stub_home.clone()).join(".groonga").join("versions");
        assert_eq!(versioned, path);
        let _ = env::set_var("HOME", home);
    }

    #[test]
    fn test_obtain_install_base_path() {
        let home = env::home_dir().unwrap();
        let stub_home = stub_home();
        let _ = env::set_var("HOME", stub_home.clone());
        let path = obtain_install_base_path();
        let versioned = PathBuf::from(stub_home.clone()).join(".groonga");
        assert_eq!(versioned, path);
        let _ = env::set_var("HOME", home);
    }
}
