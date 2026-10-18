//! A stand in for the device's secure element that refuses, the way a
//! device does when the user fails to authenticate.

use nox_shield_core::custody::HardwareGuard;
use nox_shield_core::error::CustodyError;

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
