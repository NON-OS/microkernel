//! Finding this wallet's notes in the chain's logs, each tried under the viewing key.

#[path = "logs_output.rs"]
mod output;

pub use output::output_leaves;

use super::watch::accept_incoming;
use crate::keys::Account;
use crate::notes::NoteRecord;
use std::collections::BTreeMap;

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
