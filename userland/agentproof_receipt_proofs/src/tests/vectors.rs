// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
//! A whole receipt verifies, and says what it carries.
//! Vectors: the box field's cap, and the challenge from its definition.

use agentproof_receipt::napr::read::{parse, Fault};

use crate::fixture::Fixture;

#[test]
fn a_box_word_past_255_bytes_is_refused_as_the_box_field() {
    let word = [b"allow:/tmp/".as_slice(), &[b'a'; 245]].concat();
    assert_eq!(word.len(), 256);
    let rec = Fixture::new(b"strict").record.encode();
    let bound = agentproof_receipt::napr::write::bound(&rec, &word, &[0; 256], &[0; 32]);
    let b = agentproof_receipt::napr::write::finish(bound, b"x", &[0; 45]);
    assert_eq!(parse(&b), Err(Fault::Field("box")));
}

/// The challenge, recomputed from its definition: BLAKE3 over the domain
/// and the receipt's bytes before the doc's length, nothing else.
#[test]
fn the_challenge_is_its_definition() {
    use agentproof_receipt::napr::challenge;
    let mut f = Fixture::new(b"strict");
    let bytes = f.bytes();
    let r = parse(&bytes).unwrap();
    let mut h = blake3::Hasher::new();
    h.update(b"NONOS-AGENTPROOF-RECEIPT-v0").update(&bytes[..r.bound.len()]);
    assert_eq!(challenge(r.bound), *h.finalize().as_bytes());
    assert_eq!(&bytes[r.bound.len()..r.bound.len() + 4], &(r.doc.len() as u32).to_be_bytes());
}

/// The fuzz target's seed: a whole receipt, written where cargo fuzz reads it.
#[test]
fn the_fuzz_corpus_holds_a_whole_receipt() {
    let mut f = Fixture::new(b"allow:/tmp/out,net:api.example.com:443");
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../agentproof_receipt/fuzz/corpus/napr");
    std::fs::create_dir_all(dir).unwrap();
    std::fs::write(format!("{dir}/whole"), f.bytes()).unwrap();
}
