use std::{env, fs, path::Path};

fn main() {
    println!("cargo:rerun-if-changed=../oottracker-bizhawk/OotAutoTracker/BizHawk/EmuHawk.exe");

    #[cfg(windows)]
    {
        let version_str = match winver::get_file_version_info(
            "../oottracker-bizhawk/OotAutoTracker/BizHawk/EmuHawk.exe",
        ) {
            Ok([major, minor, patch, _]) => format!("{}.{}.{}", major, minor, patch),
            Err(_) => "0.0.0".to_string(), // Fallback when BizHawk not present
        };
        fs::write(
            Path::new(&env::var("OUT_DIR").unwrap()).join("bizhawk-version.txt"),
            version_str,
        )
        .unwrap();
    }

    #[cfg(not(windows))]
    {
        // Stub version for non-Windows builds
        fs::write(
            Path::new(&env::var("OUT_DIR").unwrap()).join("bizhawk-version.txt"),
            "0.0.0",
        )
        .unwrap();
    }
}
