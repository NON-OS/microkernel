//! The one nonce the vault seals under.

use nonos_seal::NONCE_LEN;

/// The file key is fresh per vault, so a single fixed nonce is used once.
pub(super) const NONCE: [u8; NONCE_LEN] = [0u8; NONCE_LEN];
