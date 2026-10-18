//! A tampered row stops the replay.

use super::{record, scratch};
use nox_shield_core::custody::{generate_phrase, phrase_to_seed};
use nox_shield_core::error::StoreError;
use nox_shield_core::store::{NoteLog, Row};

#[test]
fn a_tampered_row_stops_the_replay() {
    let dir = scratch("store-tamper");
    let phrase = generate_phrase().expect("entropy");
    let seed = phrase_to_seed(&phrase).expect("seed");
    let path = dir.join("notes.log");
    let (mut log, _) = NoteLog::open(path.clone(), &seed).expect("open");
    log.append(&Row::Found(record(500, 0))).expect("append");

    let mut bytes = std::fs::read(&path).expect("read");
    let last = bytes.len() - 1;
    bytes[last] ^= 1;
    std::fs::write(&path, &bytes).expect("write");

    assert!(matches!(NoteLog::open(path, &seed), Err(StoreError::RowAuth)));
}
