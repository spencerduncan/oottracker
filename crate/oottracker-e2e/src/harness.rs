//! E2E test harness for coordinating tests with Project64-EM.
//!
//! This module provides a high-level test harness that coordinates:
//! - Launching and managing the emulator process
//! - Loading save states and ROM configurations
//! - Communicating with the tracker via TCP
//! - Executing test scenarios

use std::{net::SocketAddr, path::PathBuf, time::Duration};
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
    sync::mpsc,
    time::timeout,
};

use crate::launcher::{Pj64EmLauncher, Result as LauncherResult};

/// Default port for tracker communication (matches Lua harness TCP_PORT).
pub const DEFAULT_TRACKER_PORT: u16 = 24801;

/// Protocol version for handshake (matches Lua harness VERSION).
pub const PROTOCOL_VERSION: u8 = 6;

/// Packet types from the Lua harness.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PacketType {
    /// OoT RAM initialization data.
    RamInit = 4,
    /// Majora's Mask RAM initialization data.
    MmRamInit = 8,
}

impl TryFrom<u8> for PacketType {
    type Error = u8;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            4 => Ok(PacketType::RamInit),
            8 => Ok(PacketType::MmRamInit),
            other => Err(other),
        }
    }
}

/// Errors that can occur during harness operations.
#[derive(Debug, thiserror::Error)]
pub enum HarnessError {
    #[error("Launcher error: {0}")]
    Launcher(#[from] crate::launcher::LauncherError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Connection timeout")]
    ConnectionTimeout,

    #[error("Handshake failed: expected version {expected}, got {actual}")]
    HandshakeFailed { expected: u8, actual: u8 },

    #[error("Connection closed unexpectedly")]
    ConnectionClosed,

    #[error("Invalid packet type: {0}")]
    InvalidPacketType(u8),

    #[error("Test timeout: {0}")]
    TestTimeout(String),

    #[error("Scenario validation failed: {0}")]
    ValidationFailed(String),
}

/// Result type for harness operations.
pub type Result<T> = std::result::Result<T, HarnessError>;

/// Configuration for the E2E test harness.
#[derive(Debug, Clone)]
pub struct HarnessConfig {
    /// Wine prefix directory.
    pub wine_prefix: PathBuf,
    /// Path to Project64-EM executable.
    pub pj64_exe: PathBuf,
    /// ROM file to load.
    pub rom_path: PathBuf,
    /// Save state to load (optional).
    pub save_state: Option<PathBuf>,
    /// Port for tracker communication.
    pub tracker_port: u16,
    /// Timeout for emulator startup.
    pub startup_timeout: Duration,
    /// Timeout for test execution.
    pub test_timeout: Duration,
}

impl Default for HarnessConfig {
    fn default() -> Self {
        Self {
            wine_prefix: PathBuf::from(std::env::var("WINEPREFIX").unwrap_or_else(|_| {
                dirs::home_dir()
                    .map(|h| h.join(".wine").display().to_string())
                    .unwrap_or_else(|| "/home/user/.wine".to_string())
            })),
            pj64_exe: PathBuf::new(),
            rom_path: PathBuf::new(),
            save_state: None,
            tracker_port: DEFAULT_TRACKER_PORT,
            startup_timeout: Duration::from_secs(30),
            test_timeout: Duration::from_secs(60),
        }
    }
}

impl HarnessConfig {
    /// Creates a new harness configuration.
    pub fn new(wine_prefix: PathBuf, pj64_exe: PathBuf, rom_path: PathBuf) -> Self {
        Self {
            wine_prefix,
            pj64_exe,
            rom_path,
            ..Default::default()
        }
    }

    /// Sets the save state to load.
    pub fn with_save_state(mut self, save_state: PathBuf) -> Self {
        self.save_state = Some(save_state);
        self
    }

    /// Sets the tracker port.
    pub fn with_port(mut self, port: u16) -> Self {
        self.tracker_port = port;
        self
    }

    /// Sets the startup timeout.
    pub fn with_startup_timeout(mut self, timeout: Duration) -> Self {
        self.startup_timeout = timeout;
        self
    }

    /// Sets the test timeout.
    pub fn with_test_timeout(mut self, timeout: Duration) -> Self {
        self.test_timeout = timeout;
        self
    }
}

/// RAM data received from the Lua harness.
#[derive(Debug, Clone)]
pub struct RamData {
    /// Packet type.
    pub packet_type: PacketType,
    /// Raw RAM data bytes.
    pub data: Vec<u8>,
}

/// E2E test harness for coordinating tests with Project64-EM.
///
/// The harness manages:
/// - Emulator lifecycle (launch, wait, shutdown)
/// - TCP communication with the Lua harness
/// - Receiving and validating game state data
pub struct TestHarness {
    config: HarnessConfig,
    launcher: Pj64EmLauncher,
    listener: Option<TcpListener>,
    connection: Option<TcpStream>,
}

impl TestHarness {
    /// Creates a new test harness with the given configuration.
    pub fn new(config: HarnessConfig) -> Self {
        let launcher = Pj64EmLauncher::new(config.wine_prefix.clone(), config.pj64_exe.clone());

        Self {
            config,
            launcher,
            listener: None,
            connection: None,
        }
    }

    /// Starts the TCP listener for tracker communication.
    pub async fn start_listener(&mut self) -> Result<()> {
        let addr = SocketAddr::from(([127, 0, 0, 1], self.config.tracker_port));
        let listener = TcpListener::bind(addr).await?;
        self.listener = Some(listener);
        Ok(())
    }

    /// Launches the emulator and waits for it to be ready.
    pub async fn launch(&mut self) -> LauncherResult<()> {
        self.launcher.launch(&self.config.rom_path).await?;
        self.launcher
            .wait_for_ready(self.config.startup_timeout)
            .await?;
        Ok(())
    }

    /// Waits for a connection from the Lua harness.
    pub async fn wait_for_connection(&mut self) -> Result<()> {
        let listener = self.listener.as_ref().ok_or_else(|| {
            HarnessError::Io(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "Listener not started",
            ))
        })?;

        let (stream, _addr) = timeout(self.config.startup_timeout, listener.accept())
            .await
            .map_err(|_| HarnessError::ConnectionTimeout)??;

        self.connection = Some(stream);
        Ok(())
    }

    /// Performs the protocol handshake with the Lua harness.
    pub async fn handshake(&mut self) -> Result<()> {
        let stream = self.connection.as_mut().ok_or_else(|| {
            HarnessError::Io(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "Not connected",
            ))
        })?;

        let mut version_buf = [0u8; 1];
        stream.read_exact(&mut version_buf).await?;

        if version_buf[0] != PROTOCOL_VERSION {
            return Err(HarnessError::HandshakeFailed {
                expected: PROTOCOL_VERSION,
                actual: version_buf[0],
            });
        }

        Ok(())
    }

    /// Receives RAM data from the Lua harness.
    ///
    /// This method blocks until data is received or the timeout expires.
    pub async fn receive_ram_data(&mut self, timeout_duration: Duration) -> Result<RamData> {
        let stream = self.connection.as_mut().ok_or_else(|| {
            HarnessError::Io(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "Not connected",
            ))
        })?;

        let result = timeout(timeout_duration, async {
            // Read packet type
            let mut type_buf = [0u8; 1];
            stream.read_exact(&mut type_buf).await?;

            let packet_type =
                PacketType::try_from(type_buf[0]).map_err(HarnessError::InvalidPacketType)?;

            // Read remaining data (the packet length depends on RAM_RANGES configuration)
            // For now, read available data with a reasonable buffer
            let mut data = Vec::new();
            let mut buf = [0u8; 4096];

            // Read with a small timeout to get all available data
            loop {
                match tokio::time::timeout(Duration::from_millis(100), stream.read(&mut buf)).await
                {
                    Ok(Ok(0)) => break, // Connection closed
                    Ok(Ok(n)) => data.extend_from_slice(&buf[..n]),
                    Ok(Err(e)) => return Err(HarnessError::Io(e)),
                    Err(_) => break, // Timeout - no more data available
                }
            }

            Ok(RamData { packet_type, data })
        })
        .await;

        result.map_err(|_| HarnessError::TestTimeout("Waiting for RAM data".to_string()))?
    }

    /// Waits for a specific number of RAM updates.
    pub async fn wait_for_updates(&mut self, count: usize) -> Result<Vec<RamData>> {
        let mut updates = Vec::with_capacity(count);

        for _ in 0..count {
            let data = self.receive_ram_data(self.config.test_timeout).await?;
            updates.push(data);
        }

        Ok(updates)
    }

    /// Checks if the emulator is still running.
    pub fn is_running(&mut self) -> bool {
        self.launcher.is_running()
    }

    /// Shuts down the emulator.
    pub fn shutdown(&mut self) -> LauncherResult<()> {
        self.launcher.shutdown()
    }

    /// Returns the harness configuration.
    pub fn config(&self) -> &HarnessConfig {
        &self.config
    }
}

impl Drop for TestHarness {
    fn drop(&mut self) {
        // Ensure emulator is shut down
        if self.is_running() {
            let _ = self.shutdown();
        }
    }
}

/// Builder for creating test harness instances with fluent API.
#[derive(Debug, Default)]
pub struct HarnessBuilder {
    wine_prefix: Option<PathBuf>,
    pj64_exe: Option<PathBuf>,
    rom_path: Option<PathBuf>,
    save_state: Option<PathBuf>,
    tracker_port: Option<u16>,
    startup_timeout: Option<Duration>,
    test_timeout: Option<Duration>,
}

impl HarnessBuilder {
    /// Creates a new harness builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the Wine prefix directory.
    pub fn wine_prefix(mut self, path: impl Into<PathBuf>) -> Self {
        self.wine_prefix = Some(path.into());
        self
    }

    /// Sets the Project64-EM executable path.
    pub fn pj64_exe(mut self, path: impl Into<PathBuf>) -> Self {
        self.pj64_exe = Some(path.into());
        self
    }

    /// Sets the ROM file path.
    pub fn rom(mut self, path: impl Into<PathBuf>) -> Self {
        self.rom_path = Some(path.into());
        self
    }

    /// Sets the save state to load.
    pub fn save_state(mut self, path: impl Into<PathBuf>) -> Self {
        self.save_state = Some(path.into());
        self
    }

    /// Sets the tracker port.
    pub fn port(mut self, port: u16) -> Self {
        self.tracker_port = Some(port);
        self
    }

    /// Sets the startup timeout.
    pub fn startup_timeout(mut self, timeout: Duration) -> Self {
        self.startup_timeout = Some(timeout);
        self
    }

    /// Sets the test timeout.
    pub fn test_timeout(mut self, timeout: Duration) -> Self {
        self.test_timeout = Some(timeout);
        self
    }

    /// Builds the test harness.
    ///
    /// # Panics
    ///
    /// Panics if required fields (wine_prefix, pj64_exe, rom_path) are not set.
    pub fn build(self) -> TestHarness {
        let wine_prefix = self
            .wine_prefix
            .expect("wine_prefix is required for HarnessBuilder");
        let pj64_exe = self
            .pj64_exe
            .expect("pj64_exe is required for HarnessBuilder");
        let rom_path = self
            .rom_path
            .expect("rom_path is required for HarnessBuilder");

        let mut config = HarnessConfig::new(wine_prefix, pj64_exe, rom_path);

        if let Some(save_state) = self.save_state {
            config = config.with_save_state(save_state);
        }
        if let Some(port) = self.tracker_port {
            config = config.with_port(port);
        }
        if let Some(timeout) = self.startup_timeout {
            config = config.with_startup_timeout(timeout);
        }
        if let Some(timeout) = self.test_timeout {
            config = config.with_test_timeout(timeout);
        }

        TestHarness::new(config)
    }
}

/// Channel-based event receiver for async test scenarios.
pub struct EventReceiver {
    rx: mpsc::Receiver<RamData>,
}

impl EventReceiver {
    /// Receives the next RAM data event.
    pub async fn recv(&mut self) -> Option<RamData> {
        self.rx.recv().await
    }

    /// Receives the next RAM data event with a timeout.
    pub async fn recv_timeout(&mut self, timeout_duration: Duration) -> Result<RamData> {
        timeout(timeout_duration, self.rx.recv())
            .await
            .map_err(|_| HarnessError::TestTimeout("Waiting for event".to_string()))?
            .ok_or(HarnessError::ConnectionClosed)
    }
}

/// Creates an event channel for receiving RAM data asynchronously.
pub fn event_channel(buffer_size: usize) -> (mpsc::Sender<RamData>, EventReceiver) {
    let (tx, rx) = mpsc::channel(buffer_size);
    (tx, EventReceiver { rx })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harness_config_default() {
        let config = HarnessConfig::default();
        assert_eq!(config.tracker_port, DEFAULT_TRACKER_PORT);
        assert_eq!(config.startup_timeout, Duration::from_secs(30));
        assert_eq!(config.test_timeout, Duration::from_secs(60));
    }

    #[test]
    fn test_harness_config_builder() {
        let config = HarnessConfig::new(
            PathBuf::from("/home/test/.wine"),
            PathBuf::from("/home/test/pj64/Project64.exe"),
            PathBuf::from("/home/test/roms/oot.z64"),
        )
        .with_save_state(PathBuf::from("/home/test/saves/test.pj"))
        .with_port(12345)
        .with_startup_timeout(Duration::from_secs(60))
        .with_test_timeout(Duration::from_secs(120));

        assert_eq!(config.tracker_port, 12345);
        assert_eq!(config.startup_timeout, Duration::from_secs(60));
        assert_eq!(config.test_timeout, Duration::from_secs(120));
        assert!(config.save_state.is_some());
    }

    #[test]
    fn test_packet_type_conversion() {
        assert_eq!(PacketType::try_from(4), Ok(PacketType::RamInit));
        assert_eq!(PacketType::try_from(8), Ok(PacketType::MmRamInit));
        assert_eq!(PacketType::try_from(0), Err(0));
        assert_eq!(PacketType::try_from(255), Err(255));
    }

    #[test]
    fn test_harness_builder() {
        let harness = HarnessBuilder::new()
            .wine_prefix("/home/test/.wine")
            .pj64_exe("/home/test/pj64/Project64.exe")
            .rom("/home/test/roms/oot.z64")
            .port(12345)
            .build();

        assert_eq!(harness.config().tracker_port, 12345);
    }

    #[tokio::test]
    async fn test_event_channel() {
        let (tx, mut rx) = event_channel(10);

        let data = RamData {
            packet_type: PacketType::RamInit,
            data: vec![1, 2, 3, 4],
        };

        tx.send(data.clone()).await.unwrap();

        let received = rx.recv().await.unwrap();
        assert_eq!(received.packet_type, PacketType::RamInit);
        assert_eq!(received.data, vec![1, 2, 3, 4]);
    }
}
