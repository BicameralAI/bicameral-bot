//! Service installation and status checking.
//!
//! Supports systemd user units on Linux and launchd on macOS.
//! Adapted from ZeroClaw's service module.

use std::path::Path;

#[cfg(target_os = "macos")]
const SERVICE_LABEL: &str = "com.bicameral.daemon";

/// Check if the Bicameral daemon is currently running.
pub fn is_running() -> bool {
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("systemctl")
            .args(["--user", "is-active", "--quiet", "bicameral"])
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("launchctl")
            .args(["print", &format!("gui/{}/{}", get_uid(), SERVICE_LABEL)])
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        false
    }
}

/// Get the status of the Bicameral daemon service.
pub fn status() -> ServiceStatus {
    if is_running() {
        ServiceStatus::Running
    } else {
        ServiceStatus::Stopped
    }
}

/// Install the Bicameral daemon as a user-level service.
pub fn install(binary_path: &Path) -> anyhow::Result<()> {
    #[cfg(target_os = "linux")]
    {
        install_systemd(binary_path)?;
    }
    #[cfg(target_os = "macos")]
    {
        install_launchd(binary_path)?;
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        let _ = binary_path;
        anyhow::bail!("Service installation not supported on this platform");
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceStatus {
    Running,
    Stopped,
    Unknown,
}

impl std::fmt::Display for ServiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Running => write!(f, "running"),
            Self::Stopped => write!(f, "stopped"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

#[cfg(target_os = "linux")]
fn install_systemd(binary_path: &Path) -> anyhow::Result<()> {
    let unit_dir = dirs_config()
        .ok_or_else(|| anyhow::anyhow!("Cannot determine user config directory"))?
        .join("systemd/user");
    std::fs::create_dir_all(&unit_dir)?;

    let unit_content = format!(
        "[Unit]\n\
         Description=Bicameral Daemon\n\
         After=network.target\n\
         \n\
         [Service]\n\
         Type=simple\n\
         ExecStart={} gateway start\n\
         Restart=on-failure\n\
         \n\
         [Install]\n\
         WantedBy=default.target\n",
        binary_path.display()
    );

    std::fs::write(unit_dir.join("bicameral.service"), unit_content)?;

    std::process::Command::new("systemctl")
        .args(["--user", "daemon-reload"])
        .status()?;

    tracing::info!(
        "Systemd user unit installed. Enable with: systemctl --user enable --now bicameral"
    );
    Ok(())
}

#[cfg(target_os = "macos")]
fn install_launchd(binary_path: &Path) -> anyhow::Result<()> {
    let agents_dir = std::env::var("HOME")
        .map(|h| std::path::PathBuf::from(h).join("Library/LaunchAgents"))
        .map_err(|_| anyhow::anyhow!("HOME not set"))?;
    std::fs::create_dir_all(&agents_dir)?;

    let plist_content = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>{label}</string>
    <key>ProgramArguments</key>
    <array>
        <string>{binary}</string>
        <string>gateway</string>
        <string>start</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
</dict>
</plist>"#,
        label = SERVICE_LABEL,
        binary = binary_path.display()
    );

    let plist_path = agents_dir.join(format!("{}.plist", SERVICE_LABEL));
    std::fs::write(&plist_path, plist_content)?;

    tracing::info!("LaunchAgent installed at {:?}", plist_path);
    Ok(())
}

#[cfg(target_os = "macos")]
fn get_uid() -> u32 {
    unsafe { libc::getuid() }
}

#[cfg(target_os = "linux")]
fn dirs_config() -> Option<std::path::PathBuf> {
    std::env::var("XDG_CONFIG_HOME")
        .ok()
        .map(std::path::PathBuf::from)
        .or_else(|| {
            std::env::var("HOME")
                .ok()
                .map(|h| std::path::PathBuf::from(h).join(".config"))
        })
}
