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
    None
}
