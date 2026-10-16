//! Each asset's balance counts only its own notes.

use super::{record_of, scratch};
use nox_shield_core::custody::{generate_phrase, phrase_to_seed};
use nox_shield_core::net::asset::{ETH, NOX};
use nox_shield_core::store::{NoteLog, Row};

#[test]
fn each_asset_counts_only_its_own_notes() {
    let dir = scratch("store-assets");
    let seed = phrase_to_seed(&generate_phrase().expect("entropy")).expect("seed");
    let (mut log, _) = NoteLog::open(dir.join("notes.log"), &seed).expect("open");
    let rows = [
        Row::Found(record_of(ETH.id, 7_000, 0)),
        Row::Found(record_of(NOX.id, 3, 4)),
        Row::Found(record_of(NOX.id, 5, 8)),
        Row::Found(record_of(9, 1_000_000, 12)),
    ];
    for row in rows {
        log.append(&row).expect("append");
    }
    let (_reopened, state) = NoteLog::open(dir.join("notes.log"), &seed).expect("reopen");
    let (eth, nox) = (state.balance(&ETH), state.balance(&NOX));
    assert_eq!((eth.spendable, eth.note_count), (7_000, 1), "wei stays wei");
    assert_eq!((nox.spendable, nox.note_count), (8, 2), "no wei and no unknown asset in NOX");
}
