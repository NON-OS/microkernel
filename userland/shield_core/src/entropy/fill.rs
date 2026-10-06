//! The single place the core reads the platform random source: vault file key, note
//! blindings, ephemeral note keys and proof blinding seeds all come through here.

use crate::error::CustodyError;

/// Fill `out` from the kernel CSPRNG (getrandom on Android, the system device on iOS). A
/// refusal is an error: no fallback source, no time mixing, no retry, since a device without
/// randomness cannot make a hidden proof.
pub fn fill(out: &mut [u8]) -> Result<(), CustodyError> {
    getrandom::getrandom(out).map_err(|_| CustodyError::Entropy)
}
