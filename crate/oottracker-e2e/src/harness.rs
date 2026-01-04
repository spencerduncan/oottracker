//! TCP client for communicating with the E2E test harness Lua script.

use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    time::timeout,
};

/// Default port for E2E test harness (must match Lua script).
pub const E2E_HARNESS_PORT: u16 = 43435;

/// Command types sent to the Lua harness.
mod cmd {
    pub const PING: u8 = 0x01;
    pub const SAVE_STATE: u8 = 0x02;
    pub const LOAD_STATE: u8 = 0x03;
    pub const ADVANCE_FRAMES: u8 = 0x04;
    pub const READ_MEMORY: u8 = 0x05;
    pub const WRITE_MEMORY: u8 = 0x06;
    pub const GET_FRAME_COUNT: u8 = 0x07;
    pub const RESET: u8 = 0x08;
    pub const PAUSE: u8 = 0x09;
    pub const RESUME: u8 = 0x0A;
    pub const SET_INPUT: u8 = 0x0B;
}

/// Response types received from the Lua harness.
mod resp {
    pub const OK: u8 = 0x00;
    pub const ERROR: u8 = 0x01;
    pub const DATA: u8 = 0x02;
    pub const PONG: u8 = 0x03;
}

/// Errors that can occur during harness operations.
#[derive(Debug, thiserror::Error)]
pub enum HarnessError {
    #[error("Failed to connect to harness: {0}")]
    ConnectionFailed(#[source] std::io::Error),

    #[error("Connection timeout")]
    ConnectionTimeout,

    #[error("Failed to send command: {0}")]
    SendFailed(#[source] std::io::Error),

    #[error("Failed to receive response: {0}")]
    ReceiveFailed(#[source] std::io::Error),

    #[error("Unexpected response type: {0}")]
    UnexpectedResponse(u8),

    #[error("Harness error: {0}")]
    HarnessError(String),

    #[error("Operation timeout")]
    Timeout,

    #[error("Invalid data size: expected {expected}, got {actual}")]
    InvalidDataSize { expected: usize, actual: usize },
}

/// Result type for harness operations.
pub type Result<T> = std::result::Result<T, HarnessError>;

/// N64 controller button flags.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ControllerInput {
    pub a: bool,
    pub b: bool,
    pub z: bool,
    pub start: bool,
    pub d_up: bool,
    pub d_down: bool,
    pub d_left: bool,
    pub d_right: bool,
    pub l: bool,
    pub r: bool,
    pub c_up: bool,
    pub c_down: bool,
    pub c_left: bool,
    pub c_right: bool,
    /// Analog stick X axis (-128 to 127)
    pub stick_x: i8,
    /// Analog stick Y axis (-128 to 127)
    pub stick_y: i8,
}

impl ControllerInput {
    /// Converts button states to a 16-bit button mask.
    fn to_buttons(self) -> u16 {
        let mut buttons: u16 = 0;
        if self.a {
            buttons |= 0x8000;
        }
        if self.b {
            buttons |= 0x4000;
        }
        if self.z {
            buttons |= 0x2000;
        }
        if self.start {
            buttons |= 0x1000;
        }
        if self.d_up {
            buttons |= 0x0800;
        }
        if self.d_down {
            buttons |= 0x0400;
        }
        if self.d_left {
            buttons |= 0x0200;
        }
        if self.d_right {
            buttons |= 0x0100;
        }
        if self.l {
            buttons |= 0x0020;
        }
        if self.r {
            buttons |= 0x0010;
        }
        if self.c_up {
            buttons |= 0x0008;
        }
        if self.c_down {
            buttons |= 0x0004;
        }
        if self.c_left {
            buttons |= 0x0002;
        }
        if self.c_right {
            buttons |= 0x0001;
        }
        buttons
    }
}

/// Client for communicating with the E2E test harness.
///
/// This struct provides methods for controlling the emulator and accessing memory
/// through the Lua test harness script.
pub struct HarnessClient {
    stream: TcpStream,
    command_timeout: Duration,
}

impl HarnessClient {
    /// Connects to the test harness.
    ///
    /// # Arguments
    ///
    /// * `port` - The TCP port to connect to (default: 43435)
    /// * `connect_timeout` - Maximum time to wait for connection
    ///
    /// # Errors
    ///
    /// Returns an error if the connection fails or times out.
    pub async fn connect(port: u16, connect_timeout: Duration) -> Result<Self> {
        let addr = format!("127.0.0.1:{}", port);

        let stream = timeout(connect_timeout, TcpStream::connect(&addr))
            .await
            .map_err(|_| HarnessError::ConnectionTimeout)?
            .map_err(HarnessError::ConnectionFailed)?;

        Ok(Self {
            stream,
            command_timeout: Duration::from_secs(5),
        })
    }

    /// Connects to the test harness on the default port.
    pub async fn connect_default(connect_timeout: Duration) -> Result<Self> {
        Self::connect(E2E_HARNESS_PORT, connect_timeout).await
    }

    /// Sets the command timeout duration.
    pub fn set_command_timeout(&mut self, timeout: Duration) {
        self.command_timeout = timeout;
    }

    /// Sends a ping to verify the connection.
    ///
    /// Returns the harness version number.
    pub async fn ping(&mut self) -> Result<u8> {
        self.stream
            .write_all(&[cmd::PING])
            .await
            .map_err(HarnessError::SendFailed)?;

        let mut response = [0u8; 2];
        timeout(self.command_timeout, self.stream.read_exact(&mut response))
            .await
            .map_err(|_| HarnessError::Timeout)?
            .map_err(HarnessError::ReceiveFailed)?;

        if response[0] != resp::PONG {
            return Err(HarnessError::UnexpectedResponse(response[0]));
        }

        Ok(response[1])
    }

    /// Saves emulator state to the specified slot.
    ///
    /// # Arguments
    ///
    /// * `slot` - The save state slot (0-255)
    pub async fn save_state(&mut self, slot: u8) -> Result<()> {
        self.stream
            .write_all(&[cmd::SAVE_STATE, slot])
            .await
            .map_err(HarnessError::SendFailed)?;

        self.expect_ok().await
    }

    /// Loads emulator state from the specified slot.
    ///
    /// # Arguments
    ///
    /// * `slot` - The save state slot (0-255)
    pub async fn load_state(&mut self, slot: u8) -> Result<()> {
        self.stream
            .write_all(&[cmd::LOAD_STATE, slot])
            .await
            .map_err(HarnessError::SendFailed)?;

        self.expect_ok().await
    }

    /// Advances the emulator by the specified number of frames.
    ///
    /// The emulator will pause after advancing the requested frames.
    ///
    /// # Arguments
    ///
    /// * `frames` - Number of frames to advance
    pub async fn advance_frames(&mut self, frames: u32) -> Result<()> {
        let mut buf = [0u8; 5];
        buf[0] = cmd::ADVANCE_FRAMES;
        buf[1..5].copy_from_slice(&frames.to_be_bytes());

        self.stream
            .write_all(&buf)
            .await
            .map_err(HarnessError::SendFailed)?;

        self.expect_ok().await
    }

    /// Reads memory from the emulator.
    ///
    /// # Arguments
    ///
    /// * `address` - The memory address to read from (N64 address space)
    /// * `size` - Number of bytes to read (max 65536)
    ///
    /// # Returns
    ///
    /// The memory contents as a byte vector.
    pub async fn read_memory(&mut self, address: u32, size: u32) -> Result<Vec<u8>> {
        let mut buf = [0u8; 9];
        buf[0] = cmd::READ_MEMORY;
        buf[1..5].copy_from_slice(&address.to_be_bytes());
        buf[5..9].copy_from_slice(&size.to_be_bytes());

        self.stream
            .write_all(&buf)
            .await
            .map_err(HarnessError::SendFailed)?;

        self.expect_data(size as usize).await
    }

    /// Writes memory to the emulator.
    ///
    /// # Arguments
    ///
    /// * `address` - The memory address to write to (N64 address space)
    /// * `data` - The data to write
    pub async fn write_memory(&mut self, address: u32, data: &[u8]) -> Result<()> {
        let size = data.len() as u32;
        let mut header = [0u8; 9];
        header[0] = cmd::WRITE_MEMORY;
        header[1..5].copy_from_slice(&address.to_be_bytes());
        header[5..9].copy_from_slice(&size.to_be_bytes());

        self.stream
            .write_all(&header)
            .await
            .map_err(HarnessError::SendFailed)?;
        self.stream
            .write_all(data)
            .await
            .map_err(HarnessError::SendFailed)?;

        self.expect_ok().await
    }

    /// Gets the current frame count.
    pub async fn get_frame_count(&mut self) -> Result<u32> {
        self.stream
            .write_all(&[cmd::GET_FRAME_COUNT])
            .await
            .map_err(HarnessError::SendFailed)?;

        let data = self.expect_data(4).await?;
        Ok(u32::from_be_bytes([data[0], data[1], data[2], data[3]]))
    }

    /// Resets the emulator.
    pub async fn reset(&mut self) -> Result<()> {
        self.stream
            .write_all(&[cmd::RESET])
            .await
            .map_err(HarnessError::SendFailed)?;

        self.expect_ok().await
    }

    /// Pauses the emulator.
    pub async fn pause(&mut self) -> Result<()> {
        self.stream
            .write_all(&[cmd::PAUSE])
            .await
            .map_err(HarnessError::SendFailed)?;

        self.expect_ok().await
    }

    /// Resumes the emulator.
    pub async fn resume(&mut self) -> Result<()> {
        self.stream
            .write_all(&[cmd::RESUME])
            .await
            .map_err(HarnessError::SendFailed)?;

        self.expect_ok().await
    }

    /// Sets controller input for the next frame.
    ///
    /// # Arguments
    ///
    /// * `controller` - Controller number (0-3)
    /// * `input` - The controller input state
    pub async fn set_input(&mut self, controller: u8, input: ControllerInput) -> Result<()> {
        let buttons = input.to_buttons();
        let stick_x = (input.stick_x as i16 + 128) as u8;
        let stick_y = (input.stick_y as i16 + 128) as u8;

        let buf = [
            cmd::SET_INPUT,
            controller,
            (buttons >> 8) as u8,
            (buttons & 0xFF) as u8,
            stick_x,
            stick_y,
        ];

        self.stream
            .write_all(&buf)
            .await
            .map_err(HarnessError::SendFailed)?;

        self.expect_ok().await
    }

    /// Reads memory from RDRAM (with automatic base address).
    ///
    /// # Arguments
    ///
    /// * `offset` - Offset within RDRAM
    /// * `size` - Number of bytes to read
    pub async fn read_rdram(&mut self, offset: u32, size: u32) -> Result<Vec<u8>> {
        const RDRAM_BASE: u32 = 0x80000000;
        self.read_memory(RDRAM_BASE + offset, size).await
    }

    /// Writes memory to RDRAM (with automatic base address).
    ///
    /// # Arguments
    ///
    /// * `offset` - Offset within RDRAM
    /// * `data` - The data to write
    pub async fn write_rdram(&mut self, offset: u32, data: &[u8]) -> Result<()> {
        const RDRAM_BASE: u32 = 0x80000000;
        self.write_memory(RDRAM_BASE + offset, data).await
    }

    /// Waits for an OK response.
    async fn expect_ok(&mut self) -> Result<()> {
        let mut response = [0u8; 1];
        timeout(self.command_timeout, self.stream.read_exact(&mut response))
            .await
            .map_err(|_| HarnessError::Timeout)?
            .map_err(HarnessError::ReceiveFailed)?;

        match response[0] {
            resp::OK => Ok(()),
            resp::ERROR => {
                let mut len_buf = [0u8; 1];
                self.stream
                    .read_exact(&mut len_buf)
                    .await
                    .map_err(HarnessError::ReceiveFailed)?;

                let len = len_buf[0] as usize;
                let mut msg_buf = vec![0u8; len];
                self.stream
                    .read_exact(&mut msg_buf)
                    .await
                    .map_err(HarnessError::ReceiveFailed)?;

                let msg = String::from_utf8_lossy(&msg_buf).to_string();
                Err(HarnessError::HarnessError(msg))
            }
            other => Err(HarnessError::UnexpectedResponse(other)),
        }
    }

    /// Waits for a DATA response.
    async fn expect_data(&mut self, expected_size: usize) -> Result<Vec<u8>> {
        let mut response = [0u8; 5];
        timeout(self.command_timeout, self.stream.read_exact(&mut response))
            .await
            .map_err(|_| HarnessError::Timeout)?
            .map_err(HarnessError::ReceiveFailed)?;

        match response[0] {
            resp::DATA => {
                let size = u32::from_be_bytes([response[1], response[2], response[3], response[4]])
                    as usize;

                if size != expected_size {
                    return Err(HarnessError::InvalidDataSize {
                        expected: expected_size,
                        actual: size,
                    });
                }

                let mut data = vec![0u8; size];
                timeout(self.command_timeout, self.stream.read_exact(&mut data))
                    .await
                    .map_err(|_| HarnessError::Timeout)?
                    .map_err(HarnessError::ReceiveFailed)?;

                Ok(data)
            }
            resp::ERROR => {
                let mut remaining = [0u8; 1];
                self.stream
                    .read_exact(&mut remaining)
                    .await
                    .map_err(HarnessError::ReceiveFailed)?;

                let len = remaining[0] as usize;
                let mut msg_buf = vec![0u8; len];
                self.stream
                    .read_exact(&mut msg_buf)
                    .await
                    .map_err(HarnessError::ReceiveFailed)?;

                let msg = String::from_utf8_lossy(&msg_buf).to_string();
                Err(HarnessError::HarnessError(msg))
            }
            other => Err(HarnessError::UnexpectedResponse(other)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_controller_input_default() {
        let input = ControllerInput::default();
        assert_eq!(input.to_buttons(), 0);
        assert_eq!(input.stick_x, 0);
        assert_eq!(input.stick_y, 0);
    }

    #[test]
    fn test_controller_input_buttons() {
        let input = ControllerInput {
            a: true,
            b: true,
            z: true,
            start: true,
            ..Default::default()
        };
        assert_eq!(input.to_buttons(), 0xF000);
    }

    #[test]
    fn test_controller_input_c_buttons() {
        let input = ControllerInput {
            c_up: true,
            c_down: true,
            c_left: true,
            c_right: true,
            ..Default::default()
        };
        assert_eq!(input.to_buttons(), 0x000F);
    }

    #[test]
    fn test_controller_input_all_buttons() {
        let input = ControllerInput {
            a: true,
            b: true,
            z: true,
            start: true,
            d_up: true,
            d_down: true,
            d_left: true,
            d_right: true,
            l: true,
            r: true,
            c_up: true,
            c_down: true,
            c_left: true,
            c_right: true,
            stick_x: 0,
            stick_y: 0,
        };
        assert_eq!(input.to_buttons(), 0xFF3F);
    }
}
