//! Reading notes from the chain's logs, no server in the path.
//!
//! Builds the two events the pool emits for a note this wallet owns and one it
//! does not, and scans them the way the wallet scans an RPC's reply: the owned
//! note comes back, the other does not, and nothing was asked of a service.
#![allow(clippy::expect_used, clippy::indexing_slicing, clippy::panic, clippy::unwrap_used)]

use nox_shield_core::custody::{generate_phrase, phrase_to_seed};
use nox_shield_core::discovery::{scan_logs, Log};
use nox_shield_core::keys::{Account, Address};
use nox_shield_core::notes::{commitment, commitment_bytes, seal_note, wire_digest, NotePlaintext};
use nox_shield_core::prover::pool_hasher;

fn account() -> Account {
    let phrase = generate_phrase().expect("entropy");
    Account::from_seed(&phrase_to_seed(&phrase).expect("seed")).expect("account")
}

fn note_to(addr: &Address, value: u64) -> NotePlaintext {
    NotePlaintext { value, asset_id: 1, blinding: [7, 8, 9, 10], spend_pk: addr.spend_pk }
}

fn leaf_wire(plain: &NotePlaintext) -> [u8; 32] {
    let cm = commitment(&pool_hasher(), &plain.note());
    wire_digest(&[cm[0].value(), cm[1].value(), cm[2].value(), cm[3].value()])
}

/// A uint40 leaf index in the low bytes of a topic word.
fn index_topic(index: u64) -> [u8; 32] {
    let mut t = [0u8; 32];
    t[24..32].copy_from_slice(&index.to_be_bytes());
    t
}

/// One ABI `bytes`: a word of offset, a word of length, then the padded body.
fn abi_bytes(body: &[u8]) -> Vec<u8> {
    let mut out = vec![0u8; 64];
    out[31] = 0x20;
    let len = (body.len() as u64).to_be_bytes();
    out[56..64].copy_from_slice(&len);
    out.extend_from_slice(body);
    while !out.len().is_multiple_of(32) {
        out.push(0);
    }
    out
}

/// The two logs the pool emits for a note sealed to `to` at `index`.
fn logs_for(
    plain: &NotePlaintext,
    to: &Address,
    index: u64,
) -> (Vec<[u8; 32]>, Vec<[u8; 32]>, Vec<u8>) {
    let cm_bytes = commitment_bytes(&commitment(&pool_hasher(), &plain.note()));
    let cipher = seal_note(plain, to, &cm_bytes).expect("seal");
    let mut client = vec![0x01u8, cipher.view_tag];
    client.extend_from_slice(&cipher.eph_pk);
    client.extend_from_slice(&cipher.sealed);
    let ev = [0xAAu8; 32];
    let committed = vec![ev, leaf_wire(plain), index_topic(index)];
    let output = vec![ev, index_topic(index)];
    (committed, output, abi_bytes(&client))
}

#[test]
fn the_wallet_reads_its_own_note_from_the_logs() {
    let acct = account();
    let me = acct.address();
    let plain = note_to(&me, 750_000);
    let (committed, output, data) = logs_for(&plain, &me, 41);

    let nc = [Log { topics: &committed, data: &[] }];
    let on = [Log { topics: &output, data: &data }];
    let notes = scan_logs(&acct, &nc, &on);
    assert_eq!(notes.len(), 1, "the wallet's own note is read from the chain");
    assert_eq!(notes[0].plain.value, 750_000);
    assert_eq!(notes[0].leaf_index, 41);
}

#[test]
fn a_note_for_someone_else_is_not_read() {
    let acct = account();
    let other = account().address();
    let plain = note_to(&other, 750_000);
    let (committed, output, data) = logs_for(&plain, &other, 41);

    let nc = [Log { topics: &committed, data: &[] }];
    let on = [Log { topics: &output, data: &data }];
    assert!(scan_logs(&acct, &nc, &on).is_empty(), "a stranger's note is not ours");
}

#[test]
fn an_output_with_no_committed_leaf_is_skipped() {
    let acct = account();
    let me = acct.address();
    let plain = note_to(&me, 750_000);
    let (_committed, output, data) = logs_for(&plain, &me, 41);
    // The OutputNote arrived but its NoteCommitted did not: nothing to join to.
    let on = [Log { topics: &output, data: &data }];
    assert!(scan_logs(&acct, &[], &on).is_empty(), "no commitment, no note");
}

#[test]
fn a_scan_with_a_hole_is_detected() {
    use nox_shield_core::discovery::first_gap;
    use std::collections::BTreeMap;
    let mut m = BTreeMap::new();
    for i in [0u64, 1, 2, 4, 5] {
        m.insert(i, [0u8; 32]);
    }
    assert_eq!(first_gap(&m), Some(3), "leaf 3 was dropped by the RPC");
    m.insert(3, [0u8; 32]);
    assert_eq!(first_gap(&m), None, "0..6 is whole");
}
