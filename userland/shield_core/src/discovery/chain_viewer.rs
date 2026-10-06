//! The scan itself, over what a view key holds: open each output with the X-Wing key, keep what
//! commits to its leaf under `spend_pk`, match deposits sent, and with `nk` find the spent notes.

use super::chain::ChainScan;
use super::chain_read::{abi_bytes, leaf_of, record};
use super::logs::{note_commitments, Log};
use crate::keys::{nullifier_wire_with, Viewer};
use crate::notes::{
    commitment, open_xwing, wire_digest, NotePlaintext, NoteRecord, NoteStatus, XwingOpened,
};
use crate::prover::pool_hasher;
use std::collections::BTreeMap;

/// The same scan with only what a view key holds. Without `nk` no note is found spent.
pub fn scan_viewer(
    viewer: &Viewer,
    committed: &[Log],
    outputs: &[Log],
    nullifiers: &[Log],
    pending: &[NotePlaintext],
    held: &[&NoteRecord],
) -> ChainScan {
    let leaves = note_commitments(committed);
    let mut scan = ChainScan::default();
    let spend_pk = viewer.spend_pk;
    for log in outputs {
        let [_event, index] = log.topics else { continue };
        let leaf_index = leaf_of(index);
        let (Some(leaf), Some(blob)) = (leaves.get(&leaf_index), abi_bytes(log.data)) else {
            continue;
        };
        if let XwingOpened::Note(o) = open_xwing(&blob, viewer.receive.dk(), leaf, &spend_pk) {
            let plain = NotePlaintext {
                value: o.value,
                asset_id: o.asset_id,
                blinding: o.blinding,
                spend_pk,
            };
            scan.received.push(record(plain, leaf_index, leaf));
        }
    }
    let by_leaf: BTreeMap<[u8; 32], u64> = leaves.iter().map(|(i, c)| (*c, *i)).collect();
    for plain in pending {
        let cm = commitment(&pool_hasher(), &plain.note());
        let wire = wire_digest(&[cm[0].value(), cm[1].value(), cm[2].value(), cm[3].value()]);
        if let Some(index) = by_leaf.get(&wire) {
            scan.deposited.push(record(plain.clone(), *index, &wire));
        }
    }
    let Some(nk) = viewer.nk else { return scan };
    let published: Vec<[u8; 32]> =
        nullifiers.iter().filter_map(|l| l.topics.get(1).copied()).collect();
    scan.spent = held
        .iter()
        .filter(|n| n.status != NoteStatus::Spent)
        .filter(|n| published.contains(&nullifier_wire_with(nk, &n.cm, n.leaf_index)))
        .map(|n| n.cm)
        .collect();
    scan
}
