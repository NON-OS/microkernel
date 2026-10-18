//! A leaf that arrived as a ciphertext, accepted if it opens here and recomputes to the leaf.

use super::{leaf_of, words_from_wire, Leaf, WatchReject};
use crate::keys::Account;
use crate::notes::{commitment_bytes, open_note, NoteCipher, NoteRecord, NoteStatus, Opened};

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
