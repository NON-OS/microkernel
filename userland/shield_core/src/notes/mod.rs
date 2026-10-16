//! Notes: the plaintext a sender seals to a recipient, the ciphertext that
//! travels with a commitment, and the record the store keeps for a note this
//! wallet can spend.

mod blinding;
mod cipher;
mod commit;
mod kdf;
mod max_value;
mod plaintext;
mod plaintext_bytes;
mod record;
mod wire;
mod wire_digest;
mod xwing;
#[cfg(test)]
#[path = "xwing_test.rs"]
mod xwing_test;

pub use blinding::fresh_blinding;
pub use cipher::{open_note, seal_note, Opened};
pub use commit::{commitment, commitment_bytes, owner_commit_wire};
pub use max_value::MAX_VALUE;
pub use plaintext::NotePlaintext;
pub use record::{NoteRecord, NoteStatus};
pub use wire::{NoteCipher, CIPHER_LEN};
pub use wire_digest::wire_digest;
pub use xwing::{open as open_xwing, seal as seal_xwing, Opened as XwingOpened, Opening, BLOB_LEN};
