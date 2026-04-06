// Link against Tizen system libraries needed by the metadata plugin.
use std::process::Command;

fn pkg_config_libs(package: &str) {
    let output = Command::new("pkg-config")
        .args(["--libs-only-L", "--libs-only-l", package])
        .output();
    match output {
        Ok(out) if out.status.success() => {
            let flags = String::from_utf8_lossy(&out.stdout);
            for flag in flags.split_whitespace() {
                if let Some(dir) = flag.strip_prefix("-L") {
                    println!("cargo:rustc-link-search=native={}", dir);
                } else if let Some(lib) = flag.strip_prefix("-l") {
                    println!("cargo:rustc-link-lib=dylib={}", lib);
                }
            }
        }
        _ => {
            println!("cargo:rustc-link-lib=dylib=pkgmgr_installer");
            println!("cargo:rustc-link-lib=dylib=dlog");
        }
    }
}

fn main() {
    pkg_config_libs("pkgmgr-installer");
    pkg_config_libs("dlog");
}
