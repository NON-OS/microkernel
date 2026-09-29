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

//! Every vector the reference xz wrote decodes to exactly its input, and the
//! ones that must not decode do not.

#[path = "../../zstd/tests/support/kinds.rs"]
mod kinds;
#[path = "../../zstd/tests/support/rng.rs"]
mod rng;

use kinds::{mixed, noise, runs, text};
use nonos_xz::decompress;
use rng::Rng;

const TABLE: [(&str, &str, u64, usize); 15] = [
    ("empty", "text", 1, 0),
    ("tiny", "text", 2, 50),
    ("text64k", "text", 3, 65536),
    ("text300k", "text", 4, 300000),
    ("noise140k", "noise", 5, 140000),
    ("zeros300k", "zeros", 6, 300000),
    ("runs100k", "runs", 7, 100000),
    ("mixed256k", "mixed", 8, 262144),
    ("none", "text", 9, 20000),
    ("crc32", "text", 10, 20000),
    ("sha256", "text", 11, 20000),
    ("blocks", "mixed", 12, 200000),
    ("lclppb", "text", 13, 100000),
    ("lc4pb4", "runs", 14, 100000),
    ("dict4k", "text", 15, 100000),
];

fn vector(name: &str) -> Vec<u8> {
    let path = format!("{}/tests/vectors/{name}.xz", env!("CARGO_MANIFEST_DIR"));
    std::fs::read(&path).unwrap_or_else(|e| panic!("{path}: {e}"))
}

fn input(kind: &str, seed: u64, size: usize) -> Vec<u8> {
    let r = &mut Rng::new(seed);
    match kind {
        "text" => text(r, size),
        "noise" => noise(r, size),
        "runs" => runs(r, size),
        "mixed" => mixed(r, size),
        _ => vec![0; size],
    }
}

#[test]
fn every_vector_round_trips() {
    for (name, kind, seed, size) in TABLE {
        let got = decompress(&vector(name)).unwrap_or_else(|| panic!("{name}: refused"));
        assert!(got == input(kind, seed, size), "{name}: wrong bytes");
    }
}

#[test]
fn streams_and_their_padding_concatenate() {
    let mut want = text(&mut Rng::new(17), 10000);
    want.extend(noise(&mut Rng::new(18), 5000));
    assert!(decompress(&vector("concat")) == Some(want));
}

#[test]
fn a_filter_other_than_lzma2_is_refused_not_skipped() {
    assert!(decompress(&vector("bcj")).is_none());
}

#[test]
fn damage_is_caught_by_a_check_or_the_parse() {
    for name in ["text64k", "crc32", "sha256", "none"] {
        let mut v = vector(name);
        let at = v.len() / 2;
        v[at] ^= 0x10;
        assert!(decompress(&v).is_none(), "{name}");
    }
}

#[test]
fn truncation_is_refused_at_every_length() {
    let v = vector("tiny");
    for n in 0..v.len() {
        assert!(decompress(&v[..n]).is_none(), "accepted {n} of {} bytes", v.len());
    }
}
