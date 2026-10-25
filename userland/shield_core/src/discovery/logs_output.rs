//! The leaves an `OutputNote` scan yields: its client data read, each joined to its commitment.

use super::super::watch::Leaf;
use super::{leaf_index, Log};
use std::collections::BTreeMap;

/// The client-data format of `docs/17-client-data.md`: a version byte, the view tag, the ciphertext.
/// Version `0x01` (ML-KEM) is not produced yet, a gap to settle before a second wallet exists.
const CLIENT_DATA_VERSION: u8 = 0x01;
const CIPHERTEXT_LEN: usize = 128;
const CLIENT_DATA_LEN: usize = 2 + CIPHERTEXT_LEN;
/// The view tag and ciphertext of an `OutputNote`, or nothing if the length is not a note's.
fn client_data(data: &[u8]) -> Option<(u8, [u8; 128])> {
    let len_word = data.get(32..64)?;
    let mut len_bytes = [0u8; 8];
    len_bytes.copy_from_slice(len_word.get(24..32)?);
    if u64::from_be_bytes(len_bytes) as usize != CLIENT_DATA_LEN {
        return None;
    }
    let body = data.get(64..64 + CLIENT_DATA_LEN)?;
    // An unknown version is skipped, never guessed at.
    if *body.first()? != CLIENT_DATA_VERSION {
        return None;
    }
    let view_tag = *body.get(1)?;
    let mut encrypted_note = [0u8; CIPHERTEXT_LEN];
    encrypted_note.copy_from_slice(body.get(2..CLIENT_DATA_LEN)?);
    Some((view_tag, encrypted_note))
}

/// The leaves an `OutputNote` scan yields, each joined to its commitment. Malformed leaves are skipped.
pub fn output_leaves(commitments: &BTreeMap<u64, [u8; 32]>, logs: &[Log]) -> Vec<Leaf> {
    let mut out = Vec::new();
    for log in logs {
        let [_event, index] = log.topics else { continue };
        let idx = leaf_index(index);
        let Some(commitment) = commitments.get(&idx) else { continue };
        let Some((view_tag, encrypted_note)) = client_data(log.data) else { continue };
        out.push(Leaf { commitment: *commitment, leaf_index: idx, view_tag, encrypted_note });
    }
    out
}
