//! The note a payee opens, 1,186 bytes: version `0x01`, view tag, the X-Wing ciphertext
//! (ML-KEM-768 then X25519), then ChaCha20-Poly1305 of the 48-byte opening. The nonce is zero
//! because every message has its own key. The note stays sealed while either X25519 or
//! ML-KEM-768 holds, and there is no classical-only version. `xwing_test.rs` holds the pinned
//! vector. The opening must recompute to the emitted leaf, a check no pool can make.

use sha3::{Digest, Sha3_256};

#[cfg(feature = "fuzzing")]
#[path = "xwing_fuzz.rs"]
pub mod fuzz;
#[path = "xwing_opening.rs"]
mod opening;
#[path = "xwing_open.rs"]
mod reading;
#[path = "xwing_seal.rs"]
mod sealing;

pub use opening::Opening;
pub use reading::open;
pub use sealing::seal;
#[cfg(test)]
pub(crate) use sealing::seal_with;

pub const VERSION: u8 = 0x01;
pub const CIPHERTEXT_LEN: usize = 1120;
pub const OPENING_LEN: usize = 48;
const TAG_LEN: usize = 16;
/// The fixed blob length. The settlement has no room for more.
pub const BLOB_LEN: usize = 2 + CIPHERTEXT_LEN + OPENING_LEN + TAG_LEN;
const SEALED_AT: usize = 2 + CIPHERTEXT_LEN;
const TAG_AT: usize = SEALED_AT + OPENING_LEN;
const VIEW_LABEL: &[u8] = b"NOX-NOTE-VIEW-TAG-v1";
const SEAL_LABEL: &[u8] = b"NOX-NOTE-SEAL-KEY-v1";

/// How far a blob got before it was ruled out, in the order the reader tries.
pub enum Opened {
    /// Not this version's length or version, or the view tag says not ours.
    Skip,
    /// The tag matched and the seal did not open: a one-in-256 collision.
    Refuse,
    /// It opened and does not recompute to the leaf: not a note for this key.
    Discard,
    Note(Opening),
}

fn sha3(label: &[u8], ss: &[u8]) -> [u8; 32] {
    Sha3_256::new().chain_update(label).chain_update(ss).finalize().into()
}

fn aad(tag: u8, leaf: &[u8; 32]) -> [u8; 34] {
    let mut out = [0u8; 34];
    out[0] = VERSION;
    out[1] = tag;
    out[2..].copy_from_slice(leaf);
    out
}
