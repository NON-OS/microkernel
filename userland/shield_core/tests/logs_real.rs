//! The log decoder against real chain data.
//!
//! 189 `NoteCommitted` logs from the pool at 0xc0eEdE...D328 on Sepolia, the
//! genuine history the wallet must read. Untested parsing is where a reversed
//! byte order hides, so this runs the decoder over the real words and checks it
//! recovers every leaf index and commitment the chain emitted.
#![allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic,
    clippy::unwrap_used,
    clippy::arithmetic_side_effects
)]

use nox_shield_core::discovery::{note_commitments, Log};

const FIXTURE: &str = include_str!("data/notecommitted-c0eede.txt");
const EVENT: [u8; 32] = [0xAA; 32];

fn hex32(s: &str) -> [u8; 32] {
    let bytes = s.as_bytes();
    let mut out = [0u8; 32];
    for (i, slot) in out.iter_mut().enumerate() {
        let hi = (bytes[i * 2] as char).to_digit(16).expect("hex") as u8;
        let lo = (bytes[i * 2 + 1] as char).to_digit(16).expect("hex") as u8;
        *slot = (hi << 4) | lo;
    }
    out
}

#[test]
fn the_decoder_reads_every_real_leaf() {
    // Own the topic words, then borrow them into logs.
    let mut words: Vec<(u64, [u8; 32], [[u8; 32]; 3])> = Vec::new();
    for line in FIXTURE.lines() {
        let mut it = line.split_whitespace();
        let expected: u64 = it.next().expect("index").parse().expect("u64");
        let commitment = hex32(it.next().expect("commitment"));
        let index_topic = hex32(it.next().expect("index topic"));
        words.push((expected, commitment, [EVENT, commitment, index_topic]));
    }
    let logs: Vec<Log> = words.iter().map(|(_, _, t)| Log { topics: t, data: &[] }).collect();

    let map = note_commitments(&logs);
    assert_eq!(map.len(), 189, "every leaf is read");
    for (expected, commitment, _) in &words {
        assert_eq!(map.get(expected), Some(commitment), "leaf {expected} decoded wrong");
    }
    // The pool assigns leaves in order from zero, so the decoder must too.
    assert!((0..189).all(|i| map.contains_key(&i)), "the leaf indices are 0..189");
}
