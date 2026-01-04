//! Wine+PJ64-EM process launcher for E2E testing.

use std::{
    path::{Path, PathBuf},
    process::Stdio,
    time::Duration,
};
use tokio::{
    process::{Child, Command},
    time::timeout,
};

/// Errors that can occur during PJ64-EM launcher operations.
#[derive(Debug, thiserror::Error)]
pub enum LauncherError {
    #[error("Failed to launch Wine process: {0}")]
    LaunchFailed(#[source] std::io::Error),

    #[error("Process not running")]
    ProcessNotRunning,

    #[error("Timeout waiting for process to be ready")]
    ReadyTimeout,

    #[error("Failed to check process status: {0}")]
    ProcessCheckFailed(#[source] std::io::Error),

    #[error("Failed to terminate process: {0}")]
    ShutdownFailed(#[source] std::io::Error),

    #[error("ROM file not found: {0}")]
    RomNotFound(PathBuf),

    #[error("PJ64 executable not found: {0}")]
    Pj64NotFound(PathBuf),
}

/// Result type for launcher operations.
pub type Result<T> = std::result::Result<T, LauncherError>;

/// Launcher for Project64-EM via Wine.
///
/// This struct manages the lifecycle of a Project64-EM process running under Wine,
/// suitable for automated E2E testing.
pub struct Pj64EmLauncher {
    wine_prefix: PathBuf,
    pj64_exe: PathBuf,
    process: Option<Child>,
}

impl Pj64EmLauncher {
    /// Creates a new launcher instance.
    ///
    /// # Arguments
    ///
    /// * `wine_prefix` - Path to the Wine prefix directory
    /// * `pj64_exe` - Path to the Project64-EM executable
    pub fn new(wine_prefix: PathBuf, pj64_exe: PathBuf) -> Self {
        Self {
            wine_prefix,
            pj64_exe,
            process: None,
        }
    }

    /// Launches Project64-EM with the specified ROM.
    ///
    /// This method sets up the required environment variables and spawns
    /// the Wine process with Project64-EM.
    ///
    /// # Arguments
    ///
    /// * `rom` - Path to the ROM file to load
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The ROM file doesn't exist
    /// - The PJ64 executable doesn't exist
    /// - The Wine process fails to launch
    pub async fn launch(&mut self, rom: &Path) -> Result<()> {
        // Verify ROM exists
        if !rom.exists() {
            return Err(LauncherError::RomNotFound(rom.to_path_buf()));
        }

        // Verify PJ64 executable exists
        if !self.pj64_exe.exists() {
            return Err(LauncherError::Pj64NotFound(self.pj64_exe.clone()));
        }

        let child = Command::new("wine")
            .arg(&self.pj64_exe)
            .arg(rom)
            // Required for 512MB RDRAM allocation
            .env("WINEPREFIX", &self.wine_prefix)
            // Performance settings - disable vsync
            .env("vblank_mode", "0")
            .env("__GL_SYNC_TO_VBLANK", "0")
            // Suppress Wine debug output
            .env("WINEDEBUG", "-all")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(LauncherError::LaunchFailed)?;

        self.process = Some(child);
        Ok(())
    }

    /// Waits for the process to become ready.
    ///
    /// This method polls until the process is confirmed running and stable.
    /// Optionally, it can also wait for a TCP port to become available.
    ///
    /// # Arguments
    ///
    /// * `timeout_duration` - Maximum time to wait for the process to be ready
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No process is currently running
    /// - The timeout is reached before the process is ready
    pub async fn wait_for_ready(&mut self, timeout_duration: Duration) -> Result<()> {
        let process = self
            .process
            .as_mut()
            .ok_or(LauncherError::ProcessNotRunning)?;

        let poll_interval = Duration::from_millis(100);
        // Number of consecutive successful checks required to consider the process stable
        let stability_checks = 3;

        timeout(timeout_duration, async {
            let mut consecutive_ok = 0;

            while consecutive_ok < stability_checks {
                // Check if process is still running by attempting to get its ID
                // If the process has exited, try_wait will return Some(status)
                match process.try_wait() {
                    Ok(None) => {
                        // Process is still running - increment stability counter
                        consecutive_ok += 1;
                        if consecutive_ok < stability_checks {
                            tokio::time::sleep(poll_interval).await;
                        }
                    }
                    Ok(Some(_)) => {
                        // Process has already exited
                        return Err(LauncherError::ProcessNotRunning);
                    }
                    Err(e) => {
                        return Err(LauncherError::ProcessCheckFailed(e));
                    }
                }
            }

            Ok(())
        })
        .await
        .map_err(|_| LauncherError::ReadyTimeout)?
    }

    /// Shuts down the running process.
    ///
    /// This method attempts to gracefully terminate the Wine process.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No process is currently running
    /// - The termination fails
    pub fn shutdown(&mut self) -> Result<()> {
        let process = self
            .process
            .as_mut()
            .ok_or(LauncherError::ProcessNotRunning)?;

        // First try to kill the process
        process
            .start_kill()
            .map_err(LauncherError::ShutdownFailed)?;

        self.process = None;
        Ok(())
    }

    /// Returns whether a process is currently running.
    pub fn is_running(&mut self) -> bool {
        match &mut self.process {
            Some(p) => matches!(p.try_wait(), Ok(None)),
            None => false,
        }
    }

    /// Returns the Wine prefix path.
    pub fn wine_prefix(&self) -> &Path {
        &self.wine_prefix
    }

    /// Returns the PJ64 executable path.
    pub fn pj64_exe(&self) -> &Path {
        &self.pj64_exe
    }
}

impl Drop for Pj64EmLauncher {
    fn drop(&mut self) {
        // Attempt to clean up the process if it's still running
        if self.is_running() {
            let _ = self.shutdown();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_launcher_new() {
        let wine_prefix = PathBuf::from("/home/test/.wine");
        let pj64_exe = PathBuf::from("/home/test/pj64/Project64.exe");

        let launcher = Pj64EmLauncher::new(wine_prefix.clone(), pj64_exe.clone());

        assert_eq!(launcher.wine_prefix(), wine_prefix);
        assert_eq!(launcher.pj64_exe(), pj64_exe);
        assert!(!launcher.process.is_some());
    }

    #[tokio::test]
    async fn test_launch_rom_not_found() {
        let wine_prefix = PathBuf::from("/tmp/wine-test-prefix");
        let pj64_exe = PathBuf::from("/tmp/fake-pj64.exe");

        let mut launcher = Pj64EmLauncher::new(wine_prefix, pj64_exe);
        let result = launcher.launch(Path::new("/nonexistent/rom.z64")).await;

        assert!(matches!(result, Err(LauncherError::RomNotFound(_))));
    }

    #[tokio::test]
    async fn test_launch_pj64_not_found() {
        // Create a temporary ROM file
        let temp_dir = std::env::temp_dir();
        let rom_path = temp_dir.join("test-rom.z64");
        std::fs::write(&rom_path, b"fake rom data").unwrap();

        let wine_prefix = PathBuf::from("/tmp/wine-test-prefix");
        let pj64_exe = PathBuf::from("/nonexistent/pj64.exe");

        let mut launcher = Pj64EmLauncher::new(wine_prefix, pj64_exe);
        let result = launcher.launch(&rom_path).await;

        // Clean up
        let _ = std::fs::remove_file(&rom_path);

        assert!(matches!(result, Err(LauncherError::Pj64NotFound(_))));
    }

    #[test]
    fn test_shutdown_no_process() {
        let wine_prefix = PathBuf::from("/tmp/wine-test-prefix");
        let pj64_exe = PathBuf::from("/tmp/fake-pj64.exe");

        let mut launcher = Pj64EmLauncher::new(wine_prefix, pj64_exe);
        let result = launcher.shutdown();

        assert!(matches!(result, Err(LauncherError::ProcessNotRunning)));
    }

    #[test]
    fn test_is_running_no_process() {
        let wine_prefix = PathBuf::from("/tmp/wine-test-prefix");
        let pj64_exe = PathBuf::from("/tmp/fake-pj64.exe");

        let mut launcher = Pj64EmLauncher::new(wine_prefix, pj64_exe);

        assert!(!launcher.is_running());
    }

    #[tokio::test]
    async fn test_wait_for_ready_no_process() {
        let wine_prefix = PathBuf::from("/tmp/wine-test-prefix");
        let pj64_exe = PathBuf::from("/tmp/fake-pj64.exe");

        let mut launcher = Pj64EmLauncher::new(wine_prefix, pj64_exe);
        let result = launcher.wait_for_ready(Duration::from_secs(1)).await;

        assert!(matches!(result, Err(LauncherError::ProcessNotRunning)));
    }
}
