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

//! Every vector the reference encoder wrote decodes to exactly its input.

#[path = "support/kinds.rs"]
mod kinds;
#[path = "support/rng.rs"]
mod rng;
#[path = "support/table.rs"]
mod table;

use kinds::{noise, text};
use nonos_zstd::{decompress, xxh64};
use rng::Rng;
use table::{input, TABLE};

fn vector(name: &str) -> Vec<u8> {
    let path = format!("{}/tests/vectors/{name}.zst", env!("CARGO_MANIFEST_DIR"));
    std::fs::read(&path).unwrap_or_else(|e| panic!("{path}: {e}"))
}

#[test]
fn every_vector_round_trips() {
    for (name, kind, seed, size) in TABLE {
        let got = decompress(&vector(name)).unwrap_or_else(|| panic!("{name}: refused"));
        assert!(got == input(kind, seed, size), "{name}: wrong bytes");
    }
}

#[test]
fn frames_and_a_skippable_frame_concatenate() {
    let mut want = text(&mut Rng::new(13), 10000);
    want.extend(noise(&mut Rng::new(14), 5000));
    assert!(decompress(&vector("concat")) == Some(want));
}

#[test]
fn a_flipped_payload_byte_fails_the_checksum_or_the_parse() {
    let mut v = vector("text64k");
    let at = v.len() / 2;
    v[at] ^= 0x10;
    assert!(decompress(&v).is_none());
}

#[test]
fn truncation_is_refused_at_every_length() {
    let v = vector("tiny");
    for n in 0..v.len() {
        assert!(decompress(&v[..n]).is_none(), "accepted {n} of {} bytes", v.len());
    }
}

#[test]
fn xxh64_known_answers() {
    assert_eq!(xxh64(b"", 0), 0xEF46_DB37_51D8_E999);
    assert_eq!(xxh64(b"abc", 0), 0x44BC_2CF5_AD77_0999);
}
