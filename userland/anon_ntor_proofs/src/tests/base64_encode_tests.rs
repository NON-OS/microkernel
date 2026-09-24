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

//! The unpadded encoder, against RFC 4648 and the live consensus.

extern crate alloc;

use crate::directory::{decode, encode};
use crate::vectors::CONSENSUS;

/// RFC 4648 section 10, with the padding stripped, which is the form a
#[test]
fn rfc4648_vectors_without_padding() {
    let cases: &[(&[u8], &str)] = &[
        (b"", ""),
        (b"f", "Zg"),
        (b"fo", "Zm8"),
        (b"foo", "Zm9v"),
        (b"foob", "Zm9vYg"),
        (b"fooba", "Zm9vYmE"),
        (b"foobar", "Zm9vYmFy"),
    ];
    for (input, want) in cases {
        assert_eq!(encode(input), want.as_bytes(), "{want}");
    }
}

#[test]
fn every_live_m_line_round_trips() {
    let text = alloc::string::String::from_utf8_lossy(CONSENSUS);
    let mut seen = 0usize;
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("m ") {
            let digest = decode(rest.trim().as_bytes()).expect("the consensus decodes");
            assert_eq!(digest.len(), 32);
            assert_eq!(encode(&digest), rest.trim().as_bytes(), "{rest}");
            seen += 1;
        }
    }
    assert!(seen >= 6, "the vector carries m lines to check, saw {seen}");
}

#[test]
fn no_padding_is_ever_written() {
    for len in 1..40usize {
        let data: alloc::vec::Vec<u8> = (0..len as u8).collect();
        let out = encode(&data);
        assert!(!out.contains(&b'='), "padding appeared at length {len}");
        assert_eq!(decode(&out).as_deref(), Some(&data[..]), "round trip at {len}");
    }
}
