//! Note encryption. One ephemeral key agreement per note, one symmetric key
//! per note, and the commitment as associated data so a ciphertext only opens
//! beside the note it was written for.

mod open;
mod seal;

use nonos_seal::NONCE_LEN;

/// Each note key seals a single note, so its nonce is used once.
pub(super) const NONCE: [u8; NONCE_LEN] = [0u8; NONCE_LEN];

pub use open::{open_note, Opened};
pub use seal::seal_note;
