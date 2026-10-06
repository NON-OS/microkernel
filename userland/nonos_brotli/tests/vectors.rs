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

/* Every stream the reference encoder made must decode to its input,
byte for byte, and the set must span every quality and window. */

mod common;

use common::streams::cases;
use nonos_brotli::{decompress, Error};

#[test]
fn reference_streams_decode_exactly() {
    let all = cases();
    for c in &all {
        let got = decompress(c.stream, c.plain.len());
        assert_eq!(got.as_deref(), Ok(&c.plain[..]), "{}", c.name());
    }
    for q in 0..=11 {
        assert!(all.iter().any(|c| c.quality == q), "quality {q} missing");
    }
    for w in 10..=24 {
        assert!(all.iter().any(|c| c.lgwin == w), "window {w} missing");
    }
    for m in 0..=2 {
        assert!(all.iter().any(|c| c.mode == m), "mode {m} missing");
    }
}

#[test]
fn output_cap_is_exact() {
    for c in cases().iter().filter(|c| !c.plain.is_empty()) {
        let short = decompress(c.stream, c.plain.len() - 1);
        assert_eq!(short, Err(Error::TooLarge), "{}", c.name());
    }
}

#[test]
fn trailing_bytes_are_left_alone() {
    for c in cases().iter().take(8) {
        let mut padded = c.stream.to_vec();
        padded.extend_from_slice(&[0xff, 0, 0x55]);
        assert_eq!(decompress(&padded, 1 << 20).as_deref(), Ok(&c.plain[..]), "{}", c.name());
    }
}

#[test]
fn hand_made_streams() {
    /* Empty stream: WBITS 16, ISLAST, ISLASTEMPTY. */
    assert_eq!(decompress(&[0x06], 0), Ok(Vec::new()));
    /* A metadata block of two bytes, then an empty last block. */
    assert_eq!(decompress(&[0xac, 0x00, 0xaa, 0xbb, 0x03], 0), Ok(Vec::new()));
    /* One stored byte 'A' then an empty last block. */
    assert_eq!(decompress(&[0x00, 0x00, 0x10, 0x41, 0x03], 1), Ok(b"A".to_vec()));
    /* The large-window escape of WBITS is not RFC 7932. */
    assert_eq!(decompress(&[0x11, 0x00], 0), Err(Error::Invalid));
    /* Nonzero padding before stored data. */
    assert_eq!(decompress(&[0x00, 0x00, 0x30, 0x41, 0x03], 1), Err(Error::Invalid));
    assert_eq!(decompress(&[], 0), Err(Error::Truncated));
}
