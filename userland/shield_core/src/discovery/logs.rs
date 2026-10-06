//! Finding this wallet's notes in the chain's logs, each tried under the viewing key.

use super::watch::{accept_incoming, Leaf};
use crate::keys::Account;
use crate::notes::NoteRecord;
use std::collections::BTreeMap;

/// The client-data format of `docs/17-client-data.md`: a version byte, the view tag, the ciphertext.
/// Version `0x01` (ML-KEM) is not produced yet, a gap to settle before a second wallet exists.
const CLIENT_DATA_VERSION: u8 = 0x01;
const CIPHERTEXT_LEN: usize = 128;
const CLIENT_DATA_LEN: usize = 2 + CIPHERTEXT_LEN;

/// A decoded log as an RPC returns it: indexed fields as topics, the rest as data.
pub struct Log<'a> {
    pub topics: &'a [[u8; 32]],
    pub data: &'a [u8],
}

/// The leaf index a topic carries: a right aligned `uint40`, read from the low eight bytes.
fn leaf_index(topic: &[u8; 32]) -> u64 {
    let mut b = [0u8; 8];
    b.copy_from_slice(&topic[24..32]);
    u64::from_be_bytes(b)
}

/// Every leaf's commitment by index, from `NoteCommitted`.
pub fn note_commitments(logs: &[Log]) -> BTreeMap<u64, [u8; 32]> {
    let mut out = BTreeMap::new();
    for log in logs {
        if let [_event, commitment, index] = log.topics {
            out.insert(leaf_index(index), *commitment);
        }
    }
    out
}

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

/// This account's notes, read from the logs. Each is opened locally and kept only if it recomputes.
pub fn scan_logs(account: &Account, note_committed: &[Log], output: &[Log]) -> Vec<NoteRecord> {
    let commitments = note_commitments(note_committed);
    output_leaves(&commitments, output)
        .iter()
        .filter_map(|leaf| accept_incoming(account, leaf).ok())
        .collect()
}

/// The first missing leaf index. A scan with a hole is refused, a missing tail needs the pool count.
pub fn first_gap(commitments: &BTreeMap<u64, [u8; 32]>) -> Option<u64> {
    for (expected, index) in (0u64..).zip(commitments.keys()) {
        if *index != expected {
            return Some(expected);
        }
    }
    None
}
