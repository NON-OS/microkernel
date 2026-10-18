//! The one nonce a note key seals under.

use nonos_seal::NONCE_LEN;

/// Each note key seals a single note, so its nonce is used once.
pub(super) const NONCE: [u8; NONCE_LEN] = [0u8; NONCE_LEN];
