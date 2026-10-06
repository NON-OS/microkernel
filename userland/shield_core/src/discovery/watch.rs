//! Turning pool events into notes, checking each recomputes to its leaf, which the pool cannot.

use crate::keys::Account;
use crate::notes::{
    commitment, commitment_bytes, open_note, wire_digest, NoteCipher, NotePlaintext, NoteRecord,
    NoteStatus, Opened,
};
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

/// Confirm an own deposit: the escrowed value and the recomputed leaf must both match.
pub fn confirm_deposit(
    plain: &NotePlaintext,
    leaf_index: u64,
    emitted_leaf: &[u8; 32],
    emitted_value: u64,
) -> Result<NoteRecord, WatchReject> {
    if plain.value != emitted_value {
        return Err(WatchReject::Value);
    }
    let (cm, ours) = leaf_of(plain);
    if &ours != emitted_leaf {
        return Err(WatchReject::Leaf);
    }
    Ok(NoteRecord {
        plain: plain.clone(),
        leaf_index,
        cm,
        status: NoteStatus::Unspent,
        found_at: leaf_index,
    })
}

/// Accept a leaf that arrived as a ciphertext if it opens here and recomputes to the stored leaf.
pub fn accept_incoming(account: &Account, leaf: &Leaf) -> Result<NoteRecord, WatchReject> {
    let cm_words = words_from_wire(&leaf.commitment);
    let cm_bytes = commitment_bytes(&crate::discovery::words::quad(&cm_words));
    let mut eph_pk = [0u8; 32];
    eph_pk.copy_from_slice(&leaf.encrypted_note[..32]);
    let mut sealed = [0u8; 96];
    sealed.copy_from_slice(&leaf.encrypted_note[32..]);
    let cipher = NoteCipher { eph_pk, view_tag: leaf.view_tag, sealed };
    let plain = match open_note(&cipher, account.view(), &cm_bytes) {
        Opened::Note(plain) => plain,
        Opened::TagMiss | Opened::AuthFail => return Err(WatchReject::NotOurs),
    };
    // The AAD binds the ciphertext to this leaf. Recomputing proves the plaintext commits to it.
    let (_, ours) = leaf_of(&plain);
    if ours != leaf.commitment {
        return Err(WatchReject::Leaf);
    }
    Ok(NoteRecord {
        plain,
        leaf_index: leaf.leaf_index,
        cm: cm_words,
        status: NoteStatus::Unspent,
        found_at: leaf.leaf_index,
    })
}
