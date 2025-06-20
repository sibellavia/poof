use crate::constants::*;
use crate::utils::string;

#[cfg(not(target_os = "windows"))]
const ENV_PATH_SEPARATOR: &str = ":";
#[cfg(target_os = "windows")]
const ENV_PATH_SEPARATOR: &str = ";";

/// Return the separator used in the PATH environment variable.
pub fn env_path_separator() -> &'static str {
    ENV_PATH_SEPARATOR
}

/// Returns a static string containing the version information.
/// It uses Box::leak to convert a String into a &'static str.
/// This is a workaround to avoid using a global static variable.
pub fn long_version() -> &'static str {
    #[cfg(static_linking)]
    let linking_type = "statically linked";
    #[cfg(dynamic_linking)]
    let linking_type = "dynamically linked";
    Box::leak(
        format!(
            "\nVersion   : {}\nCommit    : {}\nBuild Date: {}\nC library : {} ({})",
            VERSION, COMMIT, BUILD_DATE, COMPILE_C_LIB, linking_type
        )
        .into_boxed_str(),
    )
}

pub fn short_description() -> &'static str {
    DESCRIPTION
}

pub fn get_env_var(var: &str) -> String {
    std::env::var(var).unwrap_or_else(|_| UNKNOWN.to_string())
}

pub fn get_os_version() -> String {
    if cfg!(target_os = "linux") {
        // Try to detect Linux distribution and version
        std::process::Command::new("sh")
            .arg("-c")
            .arg("(lsb_release -ds 2>/dev/null) || (cat /etc/os-release | grep PRETTY_NAME | cut -d '=' -f 2 | tr -d '\"')")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| UNKNOWN.to_string())
    } else if cfg!(target_os = "macos") {
        // Get macOS version
        std::process::Command::new("sw_vers")
            .arg("-productVersion")
            .output()
            .map(|o| format!("macOS {}", String::from_utf8_lossy(&o.stdout).trim()))
            .unwrap_or_else(|_| UNKNOWN.to_string())
    } else if cfg!(target_os = "windows") {
        // Get Windows version
        std::process::Command::new("cmd")
            .args(["/c", "ver"])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| UNKNOWN.to_string())
    } else {
        UNKNOWN.to_string()
    }
}

pub fn get_platform_endianness() -> String {
    (if cfg!(target_endian = "little") {
        "Little Endian"
    } else if cfg!(target_endian = "big") {
        "Big Endian"
    } else {
        "Unknown Endian"
    })
    .to_string()
}

pub fn get_shell_info() -> String {
    let shell_name = get_env_var("SHELL");
    let shell_version = if shell_name != UNKNOWN {
        std::process::Command::new(&shell_name)
            .arg("--version")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| UNKNOWN.to_string())
    } else {
        UNKNOWN.to_string()
    };
    format!("{} version: {}", shell_name, shell_version)
}

pub fn check_dir_in_path(dir: &str) -> i16 {
    let path = get_env_var("PATH");
    let sep = env_path_separator();
    string::position_of_str_in_string(path, sep, dir)
}
