//! Note encryption. One ephemeral key agreement per note, one symmetric key
//! per note, and the commitment as associated data so a ciphertext only opens
//! beside the note it was written for.

mod nonce;
mod open;
mod seal;

use nonce::NONCE;

pub use open::{open_note, Opened};
pub use seal::seal_note;
