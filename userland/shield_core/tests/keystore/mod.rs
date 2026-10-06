//! The keystore a test runs against, and the directory it writes into. The real guard is the
//! device's secure element. These stand in for it: one wraps faithfully, one refuses the way a
//! device does when the user fails to authenticate.

// Each test binary links this module whole, so items one binary skips are dead code there.
#![allow(dead_code)]
// Tests assert by panicking and print measurements, so those lints are off here only.
#![allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

use nox_shield_core::custody::HardwareGuard;
use nox_shield_core::error::CustodyError;

/// A stand in for the platform keystore. It hides nothing, which is fine for tests of the
/// layers above it.
pub struct TestGuard;

impl HardwareGuard for TestGuard {
    fn wrap(&self, key: Vec<u8>) -> Result<Vec<u8>, CustodyError> {
        Ok(key)
    }

    fn unwrap_key(&self, blob: Vec<u8>) -> Result<Vec<u8>, CustodyError> {
        Ok(blob)
    }
}

/// A guard that refuses, for the path where the user fails to authenticate.
pub struct RefusingGuard;

impl HardwareGuard for RefusingGuard {
    fn wrap(&self, _key: Vec<u8>) -> Result<Vec<u8>, CustodyError> {
        Err(CustodyError::Guard)
    }

    fn unwrap_key(&self, _blob: Vec<u8>) -> Result<Vec<u8>, CustodyError> {
        Err(CustodyError::Guard)
    }
}

/// A scratch directory per test name, so the suite leaves nothing in a home directory.
pub fn scratch(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join("nox-shield-tests").join(name);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a writable scratch directory");
    dir
}
