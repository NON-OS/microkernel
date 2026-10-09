//! The wallet's notes from the format 5 pool's three events.
#![allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic,
    clippy::unwrap_used,
    clippy::arithmetic_side_effects
)]

use nox_shield_core::custody::{generate_phrase, phrase_to_seed};
use nox_shield_core::discovery::{scan_chain, Log};
use nox_shield_core::keys::{
    note_nullifier_wire, parse_receiving_address, receiving_address_text, Account,
};
use nox_shield_core::notes::{
    commitment, seal_xwing, wire_digest, NotePlaintext, NoteRecord, NoteStatus, Opening,
};
use nox_shield_core::prover::pool_hasher;

fn account() -> Account {
    Account::from_seed(&phrase_to_seed(&generate_phrase().expect("entropy")).expect("seed"))
        .expect("account")
}

fn leaf_of(plain: &NotePlaintext) -> [u8; 32] {
    let cm = commitment(&pool_hasher(), &plain.note());
    wire_digest(&[cm[0].value(), cm[1].value(), cm[2].value(), cm[3].value()])
}

fn index_topic(i: u64) -> [u8; 32] {
    let mut t = [0u8; 32];
    t[24..].copy_from_slice(&i.to_be_bytes());
    t
}

fn abi_bytes(body: &[u8]) -> Vec<u8> {
    let mut out = vec![0u8; 64];
    out[31] = 0x20;
    out[56..64].copy_from_slice(&(body.len() as u64).to_be_bytes());
    out.extend_from_slice(body);
    while !out.len().is_multiple_of(32) {
        out.push(0);
    }
    out
}

/// A note paid to `payee` through their text address, as a settlement emits it.
fn paid_to(payee: &Account, value: u64) -> (NotePlaintext, Vec<u8>) {
    let (spend_pk, ek) =
        parse_receiving_address(&receiving_address_text(&payee.receiving_address())).expect("addr");
    let o = Opening { value, asset_id: 1, blinding: [value, 2, 3, 4] };
    let plain = NotePlaintext { value, asset_id: 1, blinding: o.blinding, spend_pk };
    let ek = x_wing::EncapsulationKey::try_from(ek.as_slice()).expect("ek");
    let blob = seal_xwing(&o, &ek, &leaf_of(&plain)).expect("seal");
    (plain, blob.to_vec())
}

const EV: [u8; 32] = [0xEE; 32];

#[test]
fn received_deposited_and_spent_are_all_found() {
    let me = account();
    let stranger = account();
    let (to_me, blob_me) = paid_to(&me, 500);
    let (_to_them, blob_them) = paid_to(&stranger, 900);
    // A deposit this wallet sent, and one it sent that the pool has not stored.
    let deposit = NotePlaintext {
        value: 3_000,
        asset_id: 1,
        blinding: [5, 5, 5, 5],
        spend_pk: me.address().spend_pk,
    };
    let waiting = NotePlaintext {
        value: 7_000,
        asset_id: 1,
        blinding: [6, 6, 6, 6],
        spend_pk: me.address().spend_pk,
    };

    let committed_topics = [
        [EV, leaf_of(&deposit), index_topic(0)],
        [EV, leaf_of(&to_me), index_topic(1)],
        [EV, [0x77; 32], index_topic(2)],
    ];
    let committed: Vec<Log> =
        committed_topics.iter().map(|t| Log { topics: t, data: &[] }).collect();
    let out_me = abi_bytes(&blob_me);
    let out_them = abi_bytes(&blob_them);
    let out_topics = [[EV, index_topic(1)], [EV, index_topic(2)]];
    let outputs = vec![
        Log { topics: &out_topics[0], data: &out_me },
        Log { topics: &out_topics[1], data: &out_them },
    ];

    // A note this wallet held earlier, now spent on chain.
    let held_plain = NotePlaintext {
        value: 42,
        asset_id: 1,
        blinding: [1, 1, 1, 1],
        spend_pk: me.address().spend_pk,
    };
    let held_cm = leaf_of(&held_plain);
    let mut le = held_cm;
    le.reverse();
    let mut words = [0u64; 4];
    for (w, c) in words.iter_mut().zip(le.chunks_exact(8)) {
        *w = u64::from_le_bytes(c.try_into().unwrap());
    }
    let held = [NoteRecord {
        plain: held_plain,
        leaf_index: 9,
        cm: words,
        status: NoteStatus::Unspent,
        found_at: 9,
    }];
    let nf_topics = [[EV, note_nullifier_wire(&me, &words, 9)]];
    let nullifiers = vec![Log { topics: &nf_topics[0], data: &[] }];

    let scan = scan_chain(
        &me,
        &committed,
        &outputs,
        &nullifiers,
        &[deposit, waiting],
        &held.iter().collect::<Vec<_>>(),
    );
    assert_eq!(scan.received.len(), 1, "the note paid to us, and not the stranger's");
    assert_eq!((scan.received[0].plain.value, scan.received[0].leaf_index), (500, 1));
    assert_eq!(scan.deposited.len(), 1, "the stored deposit, and not the waiting one");
    assert_eq!((scan.deposited[0].plain.value, scan.deposited[0].leaf_index), (3_000, 0));
    assert_eq!(scan.spent, vec![words], "the held note's nullifier was published");
}

#[test]
fn a_blob_without_a_committed_leaf_is_not_money() {
    let me = account();
    let (_p, blob) = paid_to(&me, 500);
    let data = abi_bytes(&blob);
    let t = [EV, index_topic(1)];
    let scan = scan_chain(&me, &[], &[Log { topics: &t, data: &data }], &[], &[], &[]);
    assert!(scan.received.is_empty());
}

#[test]
fn a_tampered_blob_is_not_money() {
    let me = account();
    let (plain, mut blob) = paid_to(&me, 500);
    let at = 1500 % blob.len();
    blob[at] ^= 1;
    let ct = [[EV, leaf_of(&plain), index_topic(1)]];
    let data = abi_bytes(&blob);
    let ot = [EV, index_topic(1)];
    let scan = scan_chain(
        &me,
        &[Log { topics: &ct[0], data: &[] }],
        &[Log { topics: &ot, data: &data }],
        &[],
        &[],
        &[],
    );
    assert!(scan.received.is_empty());
}
