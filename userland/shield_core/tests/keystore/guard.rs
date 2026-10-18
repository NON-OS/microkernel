//! A stand in for the device's secure element that wraps faithfully.

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
