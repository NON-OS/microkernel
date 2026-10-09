//! The boundary between the core and the platform keystore.
//! The core owns the seal and the platform owns the key that wraps its key, so neither side
//! opens the vault alone. There is no software fallback.

use crate::error::CustodyError;

/// The platform keystore: StrongBox on Android and the Secure Enclave on iOS.
/// Its key is generated in the secure element, never exported, and bound to user presence.
pub trait HardwareGuard: Send + Sync {
    /// Wrap a 32 byte file key. The blob is stored beside the sealed seed.
    fn wrap(&self, key: Vec<u8>) -> Result<Vec<u8>, CustodyError>;

    /// Unwrap a blob produced by `wrap` on this device.
    fn unwrap_key(&self, blob: Vec<u8>) -> Result<Vec<u8>, CustodyError>;
}
