use std::path::PathBuf;

const TIZEN_DASHBOARD_PORT: u16 = 9090;
const HOST_DASHBOARD_PORT: u16 = 9091;

pub fn is_tizen_runtime() -> bool {
    std::path::Path::new("/etc/tizen-release").exists()
        || std::path::Path::new("/opt/usr/share/tizenclaw").exists()
}

pub fn default_data_dir() -> PathBuf {
    if let Ok(path) = std::env::var("TIZENCLAW_DATA_DIR") {
        return PathBuf::from(path);
    }
    if is_tizen_runtime() {
        return PathBuf::from("/opt/usr/share/tizenclaw");
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    PathBuf::from(home).join(".tizenclaw")
}

fn default_dashboard_port_for_runtime(is_tizen_runtime: bool) -> u16 {
    if is_tizen_runtime {
        TIZEN_DASHBOARD_PORT
    } else {
        HOST_DASHBOARD_PORT
    }
}

pub fn default_dashboard_port() -> u16 {
    default_dashboard_port_for_runtime(is_tizen_runtime())
}

pub fn default_dashboard_base_url() -> String {
    format!("http://localhost:{}", default_dashboard_port())
}

pub fn default_tools_dir() -> PathBuf {
    if let Ok(path) = std::env::var("TIZENCLAW_TOOLS_DIR") {
        return PathBuf::from(path);
    }
    default_data_dir().join("tools")
}

#[cfg(test)]
mod tests {
    use super::{
        HOST_DASHBOARD_PORT, TIZEN_DASHBOARD_PORT, default_dashboard_base_url,
        default_dashboard_port_for_runtime,
    };

    #[test]
    fn default_dashboard_port_uses_tizen_default_on_tizen_runtime() {
        assert_eq!(
            default_dashboard_port_for_runtime(true),
            TIZEN_DASHBOARD_PORT
        );
    }

    #[test]
    fn default_dashboard_port_uses_ubuntu_default_on_host_runtime() {
        assert_eq!(
            default_dashboard_port_for_runtime(false),
            HOST_DASHBOARD_PORT
        );
    }

    #[test]
    fn default_dashboard_base_url_uses_localhost_with_default_port() {
        let url = default_dashboard_base_url();

        assert!(
            url == format!("http://localhost:{}", TIZEN_DASHBOARD_PORT)
                || url == format!("http://localhost:{}", HOST_DASHBOARD_PORT)
        );
    }
}
