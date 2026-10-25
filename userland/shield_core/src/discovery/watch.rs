//! Turning pool events into notes, checking each recomputes to its leaf, which the pool cannot.

#[path = "watch_deposit.rs"]
mod deposit;
#[path = "watch_incoming.rs"]
mod incoming;

pub use deposit::confirm_deposit;
pub use incoming::accept_incoming;

use crate::notes::{commitment, wire_digest, NotePlaintext};
use crate::prover::pool_hasher;

/// Why a leaf did not become a spendable note.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WatchReject {
    /// The value the pool escrowed is not the value the note was sealed to.
    Value,
    /// The stored leaf is not what the note recomputes to. Such a note is never counted as money.
    Leaf,
    /// The ciphertext did not open as this account's note.
    NotOurs,
}

/// A leaf as the pool emitted it: commitment, position, view tag and ciphertext.
pub struct Leaf {
    /// The commitment as the pool holds it, word zero lowest.
    pub commitment: [u8; 32],
    pub leaf_index: u64,
    pub view_tag: u8,
    /// The ephemeral key then the sealed plaintext: 128 bytes.
    pub encrypted_note: [u8; 128],
}

/// The pool's `bytes32` back to the wallet's four little-endian words, by reversing it.
fn words_from_wire(wire: &[u8; 32]) -> [u64; 4] {
    let mut le = *wire;
    le.reverse();
    let mut out = [0u64; 4];
    for (slot, chunk) in out.iter_mut().zip(le.chunks_exact(8)) {
        let mut w = [0u8; 8];
        w.copy_from_slice(chunk);
        *slot = u64::from_le_bytes(w);
    }
    out
}

/// The wire commitment of a note the wallet holds.
fn leaf_of(plain: &NotePlaintext) -> ([u64; 4], [u8; 32]) {
    let cm = commitment(&pool_hasher(), &plain.note());
    let words = [cm[0].value(), cm[1].value(), cm[2].value(), cm[3].value()];
    (words, wire_digest(&words))
}
