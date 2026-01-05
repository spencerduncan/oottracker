//! RAM validation test harness for emulator testing.
//!
//! This module provides tools for reading and validating RAM contents from
//! emulators running OoT/MM via the E2E test harness. It extends the base
//! [`TestHarness`](crate::harness::TestHarness) with RAM reading capabilities
//! and provides structured comparison against expected fixture values.
//!
//! # Protocol
//!
//! Communication with the Lua harness uses the following command:
//! - `CMD_READ_RAM` (0x21): Read a block of RAM
//!   - Request: 4 bytes address (big-endian), 2 bytes size (big-endian)
//!   - Response: RESP_OK (0x00) followed by data bytes
//!
//! # Example
//!
//! ```ignore
//! use oottracker_e2e::{
//!     ram_validation::{RamValidator, ValidationReport},
//!     fixtures::deku_tree_complete,
//!     HarnessBuilder,
//! };
//!
//! // Create a test harness and connect to emulator
//! let mut harness = HarnessBuilder::new()
//!     .wine_prefix("/home/user/.wine")
//!     .pj64_exe("/path/to/Project64.exe")
//!     .rom("/path/to/rom.z64")
//!     .build();
//!
//! // Create validator with expected fixture
//! let fixture = deku_tree_complete();
//! let validator = RamValidator::from_fixture(&fixture);
//!
//! // Read RAM and validate
//! let report = validator.validate(&mut harness).await?;
//! println!("Validation passed: {}", report.passed());
//! ```

use std::{collections::HashMap, fmt, time::Duration};

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::{
    fixtures::GameStateFixture,
    harness::{HarnessError, Result, TestHarness},
};

// ============================================================================
// Protocol Constants
// ============================================================================

/// Command to read RAM from the emulator.
pub const CMD_READ_RAM: u8 = 0x21;

/// Response code indicating success.
pub const RESP_OK: u8 = 0x00;

/// Response code indicating an error.
pub const RESP_ERROR: u8 = 0x01;

/// Default timeout for RAM read operations.
pub const DEFAULT_READ_TIMEOUT: Duration = Duration::from_secs(5);

/// Default port for E2E test control (matches Lua harness E2E_TEST_PORT).
pub const E2E_TEST_PORT: u16 = 24802;

// ============================================================================
// OoT Memory Addresses (from oottracker::ram)
// ============================================================================

/// OoT save context base address in RDRAM.
pub const OOT_SAVE_ADDR: u32 = 0x11A5D0;

/// OoT save context size.
pub const OOT_SAVE_SIZE: u32 = 0x1450;

/// MM save context base address in RDRAM.
pub const MM_SAVE_ADDR: u32 = 0x1EF670;

/// MM save context size.
pub const MM_SAVE_SIZE: u32 = 0x48D0;

// ============================================================================
// RAM Read Request/Response
// ============================================================================

/// A request to read RAM from the emulator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RamReadRequest {
    /// Base address to read from (RDRAM offset, not including 0x80000000).
    pub address: u32,
    /// Number of bytes to read.
    pub size: u16,
    /// Optional name/description for this read.
    pub name: Option<String>,
}

impl RamReadRequest {
    /// Creates a new RAM read request.
    pub fn new(address: u32, size: u16) -> Self {
        Self {
            address,
            size,
            name: None,
        }
    }

    /// Creates a RAM read request with a descriptive name.
    pub fn named(address: u32, size: u16, name: impl Into<String>) -> Self {
        Self {
            address,
            size,
            name: Some(name.into()),
        }
    }

    /// Encodes this request into the wire protocol format.
    ///
    /// Format: [CMD_READ_RAM] [payload_len_hi] [payload_len_lo] [addr0] [addr1] [addr2] [addr3] [size_hi] [size_lo]
    pub fn encode(&self) -> Vec<u8> {
        let payload_len: u16 = 6; // 4 bytes address + 2 bytes size
        vec![
            // Command byte
            CMD_READ_RAM,
            // Payload length (big-endian)
            (payload_len >> 8) as u8,
            payload_len as u8,
            // Address (big-endian)
            (self.address >> 24) as u8,
            (self.address >> 16) as u8,
            (self.address >> 8) as u8,
            self.address as u8,
            // Size (big-endian)
            (self.size >> 8) as u8,
            self.size as u8,
        ]
    }

    /// Creates a request for the OoT save context.
    pub fn oot_save_context() -> Self {
        Self::named(OOT_SAVE_ADDR, OOT_SAVE_SIZE as u16, "OoT Save Context")
    }

    /// Creates a request for the MM save context.
    pub fn mm_save_context() -> Self {
        Self::named(MM_SAVE_ADDR, MM_SAVE_SIZE as u16, "MM Save Context")
    }

    /// Creates a request for a specific OoT field.
    pub fn oot_field(offset: u32, size: u16, name: impl Into<String>) -> Self {
        Self::named(OOT_SAVE_ADDR + offset, size, name)
    }
}

/// Response from a RAM read operation.
#[derive(Debug, Clone)]
pub struct RamReadResponse {
    /// The original request.
    pub request: RamReadRequest,
    /// The data read from RAM, or None if the read failed.
    pub data: Option<Vec<u8>>,
    /// Error message if the read failed.
    pub error: Option<String>,
}

impl RamReadResponse {
    /// Returns true if the read was successful.
    pub fn is_ok(&self) -> bool {
        self.data.is_some()
    }

    /// Returns the data if available.
    pub fn data(&self) -> Option<&[u8]> {
        self.data.as_deref()
    }
}

// ============================================================================
// Expected Value Specifications
// ============================================================================

/// Specifies an expected value at a memory location.
#[derive(Debug, Clone)]
pub struct ExpectedValue {
    /// Address offset within the save context.
    pub offset: u32,
    /// Expected bytes at this location.
    pub expected: Vec<u8>,
    /// Human-readable field name.
    pub field_name: String,
    /// Whether this is a critical field (failure should fail the whole test).
    pub critical: bool,
}

impl ExpectedValue {
    /// Creates a new expected value specification.
    pub fn new(offset: u32, expected: Vec<u8>, field_name: impl Into<String>) -> Self {
        Self {
            offset,
            expected,
            field_name: field_name.into(),
            critical: true,
        }
    }

    /// Creates a non-critical expected value.
    pub fn optional(offset: u32, expected: Vec<u8>, field_name: impl Into<String>) -> Self {
        Self {
            offset,
            expected,
            field_name: field_name.into(),
            critical: false,
        }
    }

    /// Creates an expected value for a single byte.
    pub fn byte(offset: u32, value: u8, field_name: impl Into<String>) -> Self {
        Self::new(offset, vec![value], field_name)
    }

    /// Creates an expected value for a big-endian u16.
    pub fn u16_be(offset: u32, value: u16, field_name: impl Into<String>) -> Self {
        Self::new(offset, vec![(value >> 8) as u8, value as u8], field_name)
    }

    /// Creates an expected value for a big-endian u32.
    pub fn u32_be(offset: u32, value: u32, field_name: impl Into<String>) -> Self {
        Self::new(
            offset,
            vec![
                (value >> 24) as u8,
                (value >> 16) as u8,
                (value >> 8) as u8,
                value as u8,
            ],
            field_name,
        )
    }
}

/// Comparison mode for validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompareMode {
    /// Exact byte-for-byte comparison.
    #[default]
    Exact,
    /// Check that expected bits are set (mask comparison).
    BitsSet,
    /// Check that expected bits are clear.
    BitsClear,
}

// ============================================================================
// Validation Results
// ============================================================================

/// Result of comparing a single field.
#[derive(Debug, Clone)]
pub struct FieldResult {
    /// The expected value specification.
    pub expected: ExpectedValue,
    /// The actual bytes read from RAM.
    pub actual: Vec<u8>,
    /// Whether the comparison passed.
    pub passed: bool,
    /// Comparison mode used.
    pub mode: CompareMode,
}

impl FieldResult {
    /// Creates a passed result.
    pub fn pass(expected: ExpectedValue, actual: Vec<u8>, mode: CompareMode) -> Self {
        Self {
            expected,
            actual,
            passed: true,
            mode,
        }
    }

    /// Creates a failed result.
    pub fn fail(expected: ExpectedValue, actual: Vec<u8>, mode: CompareMode) -> Self {
        Self {
            expected,
            actual,
            passed: false,
            mode,
        }
    }
}

impl fmt::Display for FieldResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.passed { "PASS" } else { "FAIL" };
        write!(
            f,
            "[{}] {} @ 0x{:04X}: expected {:02X?}, got {:02X?}",
            status,
            self.expected.field_name,
            self.expected.offset,
            self.expected.expected,
            self.actual
        )
    }
}

/// Overall validation report.
#[derive(Debug, Clone)]
pub struct ValidationReport {
    /// Name/description of this validation run.
    pub name: String,
    /// Individual field results.
    pub results: Vec<FieldResult>,
    /// Time taken for the validation.
    pub duration: Duration,
    /// Any errors that occurred during reading.
    pub errors: Vec<String>,
}

impl ValidationReport {
    /// Creates a new empty validation report.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            results: Vec::new(),
            duration: Duration::ZERO,
            errors: Vec::new(),
        }
    }

    /// Returns true if all validations passed (no critical failures).
    pub fn passed(&self) -> bool {
        self.errors.is_empty()
            && self
                .results
                .iter()
                .filter(|r| r.expected.critical)
                .all(|r| r.passed)
    }

    /// Returns the number of passed validations.
    pub fn pass_count(&self) -> usize {
        self.results.iter().filter(|r| r.passed).count()
    }

    /// Returns the number of failed validations.
    pub fn fail_count(&self) -> usize {
        self.results.iter().filter(|r| !r.passed).count()
    }

    /// Returns the number of critical failures.
    pub fn critical_fail_count(&self) -> usize {
        self.results
            .iter()
            .filter(|r| !r.passed && r.expected.critical)
            .count()
    }

    /// Returns all failed field results.
    pub fn failures(&self) -> Vec<&FieldResult> {
        self.results.iter().filter(|r| !r.passed).collect()
    }

    /// Adds a field result to the report.
    pub fn add_result(&mut self, result: FieldResult) {
        self.results.push(result);
    }

    /// Adds an error to the report.
    pub fn add_error(&mut self, error: impl Into<String>) {
        self.errors.push(error.into());
    }

    /// Sets the duration of the validation.
    pub fn set_duration(&mut self, duration: Duration) {
        self.duration = duration;
    }

    /// Generates a summary string.
    pub fn summary(&self) -> String {
        let status = if self.passed() { "PASSED" } else { "FAILED" };
        format!(
            "{}: {} ({}/{} checks passed, {} critical failures, {} errors, {:?})",
            self.name,
            status,
            self.pass_count(),
            self.results.len(),
            self.critical_fail_count(),
            self.errors.len(),
            self.duration
        )
    }
}

impl fmt::Display for ValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== Validation Report: {} ===", self.name)?;
        writeln!(f)?;

        // Summary
        writeln!(f, "{}", self.summary())?;
        writeln!(f)?;

        // Errors
        if !self.errors.is_empty() {
            writeln!(f, "Errors:")?;
            for error in &self.errors {
                writeln!(f, "  - {}", error)?;
            }
            writeln!(f)?;
        }

        // Failed checks
        let failures: Vec<_> = self.failures();
        if !failures.is_empty() {
            writeln!(f, "Failed Checks:")?;
            for result in failures {
                writeln!(f, "  {}", result)?;
            }
            writeln!(f)?;
        }

        // All results (if verbose)
        writeln!(f, "All Results:")?;
        for result in &self.results {
            writeln!(f, "  {}", result)?;
        }

        Ok(())
    }
}

// ============================================================================
// RAM Validator
// ============================================================================

/// Validator for comparing RAM contents against expected values.
#[derive(Debug, Clone)]
pub struct RamValidator {
    /// Name of this validator.
    pub name: String,
    /// Base address for all reads (e.g., OOT_SAVE_ADDR).
    pub base_address: u32,
    /// Expected values to check.
    pub expectations: Vec<ExpectedValue>,
    /// Comparison mode.
    pub mode: CompareMode,
    /// Timeout for read operations.
    pub timeout: Duration,
}

impl RamValidator {
    /// Creates a new RAM validator.
    pub fn new(name: impl Into<String>, base_address: u32) -> Self {
        Self {
            name: name.into(),
            base_address,
            expectations: Vec::new(),
            mode: CompareMode::Exact,
            timeout: DEFAULT_READ_TIMEOUT,
        }
    }

    /// Creates a validator for OoT save context.
    pub fn oot_save() -> Self {
        Self::new("OoT Save Context", OOT_SAVE_ADDR)
    }

    /// Creates a validator for MM save context.
    pub fn mm_save() -> Self {
        Self::new("MM Save Context", MM_SAVE_ADDR)
    }

    /// Creates a validator from a game state fixture.
    ///
    /// This extracts expected values from the fixture's save context representation.
    pub fn from_fixture(fixture: &GameStateFixture) -> Self {
        let mut validator = Self::oot_save();
        validator.name = format!("Fixture: {}", fixture.id);

        // Add ZELDAZ magic check
        validator.expectations.push(ExpectedValue::new(
            0x1C,
            vec![0x5A, 0x45, 0x4C, 0x44, 0x41, 0x5A], // "ZELDAZ"
            "ZELDAZ Magic",
        ));

        // Add equipment check
        let equip_bytes = fixture.equipment.to_bytes();
        validator
            .expectations
            .push(ExpectedValue::new(0x9C, equip_bytes.to_vec(), "Equipment"));

        // Add quest status check
        let quest_bytes = fixture.quest_status.to_bytes();
        validator.expectations.push(ExpectedValue::new(
            0xA4,
            quest_bytes.to_vec(),
            "Quest Status",
        ));

        // Add health check
        validator.expectations.push(ExpectedValue::u16_be(
            0x30,
            fixture.health,
            "Current Health",
        ));
        validator.expectations.push(ExpectedValue::u16_be(
            0x2E,
            fixture.max_health,
            "Max Health",
        ));

        // Add rupees check
        validator
            .expectations
            .push(ExpectedValue::u16_be(0x34, fixture.rupees, "Rupees"));

        validator
    }

    /// Adds an expected value to check.
    pub fn expect(mut self, expected: ExpectedValue) -> Self {
        self.expectations.push(expected);
        self
    }

    /// Adds a byte expectation.
    pub fn expect_byte(mut self, offset: u32, value: u8, name: impl Into<String>) -> Self {
        self.expectations
            .push(ExpectedValue::byte(offset, value, name));
        self
    }

    /// Adds a u16 (big-endian) expectation.
    pub fn expect_u16(mut self, offset: u32, value: u16, name: impl Into<String>) -> Self {
        self.expectations
            .push(ExpectedValue::u16_be(offset, value, name));
        self
    }

    /// Adds a u32 (big-endian) expectation.
    pub fn expect_u32(mut self, offset: u32, value: u32, name: impl Into<String>) -> Self {
        self.expectations
            .push(ExpectedValue::u32_be(offset, value, name));
        self
    }

    /// Sets the comparison mode.
    pub fn with_mode(mut self, mode: CompareMode) -> Self {
        self.mode = mode;
        self
    }

    /// Sets the read timeout.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Calculates the total size needed to read all expectations.
    fn required_read_size(&self) -> u16 {
        self.expectations
            .iter()
            .map(|e| (e.offset + e.expected.len() as u32) as u16)
            .max()
            .unwrap_or(0)
    }

    /// Validates RAM contents against expectations using the provided harness.
    ///
    /// This method:
    /// 1. Reads the required RAM region from the emulator
    /// 2. Compares each expected value against the actual data
    /// 3. Returns a structured report of the results
    pub async fn validate(&self, harness: &mut TestHarness) -> Result<ValidationReport> {
        let start = std::time::Instant::now();
        let mut report = ValidationReport::new(&self.name);

        // Calculate read size and create request
        let read_size = self.required_read_size();
        if read_size == 0 {
            report.add_error("No expectations defined");
            report.set_duration(start.elapsed());
            return Ok(report);
        }

        let request = RamReadRequest::named(
            self.base_address,
            read_size,
            format!("{} (0x{:X} bytes)", self.name, read_size),
        );

        // Read RAM data
        let response = read_ram(harness, &request, self.timeout).await?;

        let data = match response.data {
            Some(data) => data,
            None => {
                report.add_error(
                    response
                        .error
                        .unwrap_or_else(|| "Unknown read error".into()),
                );
                report.set_duration(start.elapsed());
                return Ok(report);
            }
        };

        // Validate each expectation
        for expected in &self.expectations {
            let offset = expected.offset as usize;
            let len = expected.expected.len();

            // Check bounds
            if offset + len > data.len() {
                report.add_result(FieldResult::fail(expected.clone(), vec![], self.mode));
                continue;
            }

            let actual = data[offset..offset + len].to_vec();
            let passed = match self.mode {
                CompareMode::Exact => actual == expected.expected,
                CompareMode::BitsSet => actual
                    .iter()
                    .zip(&expected.expected)
                    .all(|(a, e)| (a & e) == *e),
                CompareMode::BitsClear => actual
                    .iter()
                    .zip(&expected.expected)
                    .all(|(a, e)| (a & e) == 0),
            };

            let result = if passed {
                FieldResult::pass(expected.clone(), actual, self.mode)
            } else {
                FieldResult::fail(expected.clone(), actual, self.mode)
            };

            report.add_result(result);
        }

        report.set_duration(start.elapsed());
        Ok(report)
    }

    /// Validates against raw RAM data (for testing without harness).
    pub fn validate_data(&self, data: &[u8]) -> ValidationReport {
        let start = std::time::Instant::now();
        let mut report = ValidationReport::new(&self.name);

        for expected in &self.expectations {
            let offset = expected.offset as usize;
            let len = expected.expected.len();

            if offset + len > data.len() {
                report.add_result(FieldResult::fail(expected.clone(), vec![], self.mode));
                continue;
            }

            let actual = data[offset..offset + len].to_vec();
            let passed = match self.mode {
                CompareMode::Exact => actual == expected.expected,
                CompareMode::BitsSet => actual
                    .iter()
                    .zip(&expected.expected)
                    .all(|(a, e)| (a & e) == *e),
                CompareMode::BitsClear => actual
                    .iter()
                    .zip(&expected.expected)
                    .all(|(a, e)| (a & e) == 0),
            };

            let result = if passed {
                FieldResult::pass(expected.clone(), actual, self.mode)
            } else {
                FieldResult::fail(expected.clone(), actual, self.mode)
            };

            report.add_result(result);
        }

        report.set_duration(start.elapsed());
        report
    }
}

// ============================================================================
// RAM Reading Functions
// ============================================================================

/// Reads RAM from the emulator via the test harness.
///
/// This sends a CMD_READ_RAM command to the Lua harness and waits for the response.
///
/// # Arguments
///
/// * `harness` - The test harness with an active connection
/// * `request` - The RAM read request specifying address and size
/// * `timeout` - Maximum time to wait for the response
///
/// # Errors
///
/// Returns an error if:
/// - The harness is not connected to the emulator
/// - The read operation times out
/// - There is an I/O error during communication
pub async fn read_ram(
    harness: &mut TestHarness,
    request: &RamReadRequest,
    timeout: Duration,
) -> Result<RamReadResponse> {
    let stream = harness.connection_mut().ok_or_else(|| {
        HarnessError::Io(std::io::Error::new(
            std::io::ErrorKind::NotConnected,
            "Not connected to emulator - call wait_for_connection() first",
        ))
    })?;

    // Send the request
    let request_bytes = request.encode();
    stream.write_all(&request_bytes).await?;
    stream.flush().await?;

    // Read response with timeout
    let result = tokio::time::timeout(timeout, async {
        // Read response type
        let mut resp_type = [0u8; 1];
        stream.read_exact(&mut resp_type).await?;

        if resp_type[0] == RESP_OK {
            // Read the data
            let mut data = vec![0u8; request.size as usize];
            stream.read_exact(&mut data).await?;

            Ok(RamReadResponse {
                request: request.clone(),
                data: Some(data),
                error: None,
            })
        } else {
            // Read error code
            let mut error_code = [0u8; 1];
            let _ = stream.read_exact(&mut error_code).await;

            Ok(RamReadResponse {
                request: request.clone(),
                data: None,
                error: Some(format!(
                    "Read failed with error code: 0x{:02X}",
                    error_code[0]
                )),
            })
        }
    })
    .await;

    match result {
        Ok(Ok(response)) => Ok(response),
        Ok(Err(e)) => Err(HarnessError::Io(e)),
        Err(_) => Err(HarnessError::TestTimeout(format!(
            "RAM read timed out after {:?}",
            timeout
        ))),
    }
}

/// Reads multiple RAM regions in sequence.
pub async fn read_ram_batch(
    harness: &mut TestHarness,
    requests: &[RamReadRequest],
    timeout: Duration,
) -> Result<Vec<RamReadResponse>> {
    let mut responses = Vec::with_capacity(requests.len());

    for request in requests {
        let response = read_ram(harness, request, timeout).await?;
        responses.push(response);
    }

    Ok(responses)
}

// ============================================================================
// TestHarness Extension
// ============================================================================

/// Extension trait for TestHarness to add RAM validation methods.
pub trait RamValidationExt {
    /// Reads RAM from the emulator.
    fn read_ram(
        &mut self,
        request: &RamReadRequest,
    ) -> impl std::future::Future<Output = Result<RamReadResponse>>;

    /// Validates RAM against a validator.
    fn validate_ram(
        &mut self,
        validator: &RamValidator,
    ) -> impl std::future::Future<Output = Result<ValidationReport>>;

    /// Validates RAM against a fixture.
    fn validate_fixture(
        &mut self,
        fixture: &GameStateFixture,
    ) -> impl std::future::Future<Output = Result<ValidationReport>>;
}

impl RamValidationExt for TestHarness {
    async fn read_ram(&mut self, request: &RamReadRequest) -> Result<RamReadResponse> {
        read_ram(self, request, DEFAULT_READ_TIMEOUT).await
    }

    async fn validate_ram(&mut self, validator: &RamValidator) -> Result<ValidationReport> {
        validator.validate(self).await
    }

    async fn validate_fixture(&mut self, fixture: &GameStateFixture) -> Result<ValidationReport> {
        let validator = RamValidator::from_fixture(fixture);
        validator.validate(self).await
    }
}

// ============================================================================
// Batch Validation
// ============================================================================

/// Validates multiple fixtures and generates a combined report.
#[derive(Debug, Clone)]
pub struct BatchValidator {
    /// Validators to run.
    pub validators: Vec<RamValidator>,
}

impl BatchValidator {
    /// Creates a new batch validator.
    pub fn new() -> Self {
        Self {
            validators: Vec::new(),
        }
    }

    /// Adds a validator to the batch.
    pub fn with_validator(mut self, validator: RamValidator) -> Self {
        self.validators.push(validator);
        self
    }

    /// Creates validators from multiple fixtures.
    pub fn from_fixtures(fixtures: &[GameStateFixture]) -> Self {
        let validators = fixtures.iter().map(RamValidator::from_fixture).collect();
        Self { validators }
    }

    /// Validates all validators against the current RAM state.
    ///
    /// Note: This reads RAM once per validator. For efficiency with many
    /// validators over the same region, consider using a single read and
    /// `validate_data()`.
    pub async fn validate(&self, harness: &mut TestHarness) -> Result<Vec<ValidationReport>> {
        let mut reports = Vec::with_capacity(self.validators.len());

        for validator in &self.validators {
            let report = validator.validate(harness).await?;
            reports.push(report);
        }

        Ok(reports)
    }

    /// Returns a summary of all validation reports.
    pub fn summarize(reports: &[ValidationReport]) -> BatchSummary {
        let total = reports.len();
        let passed = reports.iter().filter(|r| r.passed()).count();
        let failed = total - passed;

        let total_checks: usize = reports.iter().map(|r| r.results.len()).sum();
        let passed_checks: usize = reports.iter().map(|r| r.pass_count()).sum();
        let failed_checks = total_checks - passed_checks;

        BatchSummary {
            total_validators: total,
            passed_validators: passed,
            failed_validators: failed,
            total_checks,
            passed_checks,
            failed_checks,
        }
    }
}

impl Default for BatchValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Summary of batch validation results.
#[derive(Debug, Clone)]
pub struct BatchSummary {
    /// Total number of validators run.
    pub total_validators: usize,
    /// Number of validators that passed.
    pub passed_validators: usize,
    /// Number of validators that failed.
    pub failed_validators: usize,
    /// Total number of individual checks.
    pub total_checks: usize,
    /// Number of checks that passed.
    pub passed_checks: usize,
    /// Number of checks that failed.
    pub failed_checks: usize,
}

impl fmt::Display for BatchSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Batch Validation: {}/{} validators passed, {}/{} checks passed",
            self.passed_validators, self.total_validators, self.passed_checks, self.total_checks
        )
    }
}

// ============================================================================
// Predefined Validators
// ============================================================================

/// Creates a validator for checking ZELDAZ magic (game is running OoT).
pub fn zeldaz_validator() -> RamValidator {
    RamValidator::oot_save().expect(ExpectedValue::new(
        0x1C,
        vec![0x5A, 0x45, 0x4C, 0x44, 0x41, 0x5A],
        "ZELDAZ Magic",
    ))
}

/// Creates a validator for checking the game mode.
pub fn game_mode_validator(mode: u32) -> RamValidator {
    RamValidator::oot_save().expect(ExpectedValue::u32_be(0x135C, mode, "Game Mode"))
}

/// Creates a validator for inventory slots.
pub fn inventory_validator(inventory: HashMap<u32, u8>) -> RamValidator {
    let mut validator = RamValidator::oot_save();
    for (slot, item_id) in inventory {
        validator = validator.expect_byte(0x74 + slot, item_id, format!("Inventory Slot {}", slot));
    }
    validator
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixtures::{deku_tree_complete, ZELDAZ_MAGIC, ZELDAZ_OFFSET};

    #[test]
    fn test_ram_read_request_encode() {
        let request = RamReadRequest::new(0x11A5D0, 0x100);
        let encoded = request.encode();

        assert_eq!(encoded[0], CMD_READ_RAM);
        // Payload length = 6
        assert_eq!(encoded[1], 0x00);
        assert_eq!(encoded[2], 0x06);
        // Address = 0x11A5D0
        assert_eq!(encoded[3], 0x00);
        assert_eq!(encoded[4], 0x11);
        assert_eq!(encoded[5], 0xA5);
        assert_eq!(encoded[6], 0xD0);
        // Size = 0x100
        assert_eq!(encoded[7], 0x01);
        assert_eq!(encoded[8], 0x00);
    }

    #[test]
    fn test_expected_value_byte() {
        let expected = ExpectedValue::byte(0x10, 0x42, "Test Byte");
        assert_eq!(expected.offset, 0x10);
        assert_eq!(expected.expected, vec![0x42]);
        assert_eq!(expected.field_name, "Test Byte");
        assert!(expected.critical);
    }

    #[test]
    fn test_expected_value_u16() {
        let expected = ExpectedValue::u16_be(0x20, 0x1234, "Test U16");
        assert_eq!(expected.offset, 0x20);
        assert_eq!(expected.expected, vec![0x12, 0x34]);
    }

    #[test]
    fn test_expected_value_u32() {
        let expected = ExpectedValue::u32_be(0x30, 0x12345678, "Test U32");
        assert_eq!(expected.offset, 0x30);
        assert_eq!(expected.expected, vec![0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn test_validation_report_passed() {
        let mut report = ValidationReport::new("Test");
        report.add_result(FieldResult::pass(
            ExpectedValue::byte(0, 0x42, "Test"),
            vec![0x42],
            CompareMode::Exact,
        ));

        assert!(report.passed());
        assert_eq!(report.pass_count(), 1);
        assert_eq!(report.fail_count(), 0);
    }

    #[test]
    fn test_validation_report_failed() {
        let mut report = ValidationReport::new("Test");
        report.add_result(FieldResult::fail(
            ExpectedValue::byte(0, 0x42, "Test"),
            vec![0x99],
            CompareMode::Exact,
        ));

        assert!(!report.passed());
        assert_eq!(report.pass_count(), 0);
        assert_eq!(report.fail_count(), 1);
    }

    #[test]
    fn test_validation_report_non_critical_failure() {
        let mut report = ValidationReport::new("Test");
        report.add_result(FieldResult::fail(
            ExpectedValue::optional(0, vec![0x42], "Optional Test"),
            vec![0x99],
            CompareMode::Exact,
        ));

        // Non-critical failure should not cause overall failure
        assert!(report.passed());
        assert_eq!(report.fail_count(), 1);
        assert_eq!(report.critical_fail_count(), 0);
    }

    #[test]
    fn test_ram_validator_from_fixture() {
        let fixture = deku_tree_complete();
        let validator = RamValidator::from_fixture(&fixture);

        assert!(validator.name.contains("deku_tree_complete"));
        assert!(!validator.expectations.is_empty());

        // Should include ZELDAZ check
        assert!(validator
            .expectations
            .iter()
            .any(|e| e.field_name == "ZELDAZ Magic"));
    }

    #[test]
    fn test_ram_validator_validate_data() {
        // Create test data with ZELDAZ magic
        let mut data = vec![0u8; 0x100];
        data[ZELDAZ_OFFSET..ZELDAZ_OFFSET + 6].copy_from_slice(&ZELDAZ_MAGIC);

        let validator = zeldaz_validator();
        let report = validator.validate_data(&data);

        assert!(report.passed());
        assert_eq!(report.pass_count(), 1);
    }

    #[test]
    fn test_ram_validator_validate_data_fails() {
        // Create test data without ZELDAZ magic
        let data = vec![0u8; 0x100];

        let validator = zeldaz_validator();
        let report = validator.validate_data(&data);

        assert!(!report.passed());
        assert_eq!(report.fail_count(), 1);
    }

    #[test]
    fn test_compare_mode_bits_set() {
        let data = vec![0xFF]; // All bits set

        let validator = RamValidator::oot_save()
            .with_mode(CompareMode::BitsSet)
            .expect(ExpectedValue::byte(0, 0x0F, "Lower Nibble"));

        let report = validator.validate_data(&data);
        assert!(report.passed());
    }

    #[test]
    fn test_compare_mode_bits_clear() {
        let data = vec![0xF0]; // Upper nibble set, lower clear

        let validator = RamValidator::oot_save()
            .with_mode(CompareMode::BitsClear)
            .expect(ExpectedValue::byte(0, 0x0F, "Lower Nibble"));

        let report = validator.validate_data(&data);
        assert!(report.passed());
    }

    #[test]
    fn test_batch_summary() {
        let mut report1 = ValidationReport::new("Test 1");
        report1.add_result(FieldResult::pass(
            ExpectedValue::byte(0, 0x42, "Test"),
            vec![0x42],
            CompareMode::Exact,
        ));

        let mut report2 = ValidationReport::new("Test 2");
        report2.add_result(FieldResult::fail(
            ExpectedValue::byte(0, 0x42, "Test"),
            vec![0x99],
            CompareMode::Exact,
        ));

        let summary = BatchValidator::summarize(&[report1, report2]);

        assert_eq!(summary.total_validators, 2);
        assert_eq!(summary.passed_validators, 1);
        assert_eq!(summary.failed_validators, 1);
        assert_eq!(summary.total_checks, 2);
        assert_eq!(summary.passed_checks, 1);
        assert_eq!(summary.failed_checks, 1);
    }

    #[test]
    fn test_field_result_display() {
        let result = FieldResult::pass(
            ExpectedValue::byte(0x10, 0x42, "Test Field"),
            vec![0x42],
            CompareMode::Exact,
        );

        let display = result.to_string();
        assert!(display.contains("PASS"));
        assert!(display.contains("Test Field"));
        assert!(display.contains("0x0010"));
    }

    #[test]
    fn test_validation_report_display() {
        let mut report = ValidationReport::new("Test Report");
        report.add_result(FieldResult::pass(
            ExpectedValue::byte(0, 0x42, "Test"),
            vec![0x42],
            CompareMode::Exact,
        ));

        let display = report.to_string();
        assert!(display.contains("Test Report"));
        assert!(display.contains("PASS"));
    }

    #[test]
    fn test_oot_field_request() {
        let request = RamReadRequest::oot_field(0x1C, 6, "ZELDAZ");
        assert_eq!(request.address, OOT_SAVE_ADDR + 0x1C);
        assert_eq!(request.size, 6);
    }
}
