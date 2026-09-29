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

//! Test data under vectors/ and small builders shared by the tests.

use std::path::PathBuf;

pub use crate::bits::Bits;

pub fn path(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("vectors").join(rel)
}

pub fn read(rel: &str) -> Vec<u8> {
    std::fs::read(path(rel)).unwrap_or_else(|e| panic!("{rel}: {e}"))
}

/* The body every synthetic case carries: "cafe" with an e-acute in UTF-8. */
pub const PAGE: &[u8] = b"<!doctype html><title>t</title><p>caf\xc3\xa9 ok</p>\n";

/* A response with the given head lines and body, CRLF framed. */
pub fn response(head: &[&str], body: &[u8]) -> Vec<u8> {
    let mut r = Vec::new();
    for line in head {
        r.extend_from_slice(line.as_bytes());
        r.extend_from_slice(b"\r\n");
    }
    r.extend_from_slice(b"\r\n");
    r.extend_from_slice(body);
    r
}

/* A raw DEFLATE stream (fixed Huffman) of one zero byte then `matches`
copies of 258 zeros at distance 1: 13 bits per 258 output bytes, the
shape of a decompression bomb. */
pub fn zeros_deflate(matches: usize) -> Vec<u8> {
    let mut w = Bits::default();
    w.put(1, 1);
    w.put(1, 2);
    w.put(0x30u32.reverse_bits() >> 24, 8);
    for _ in 0..matches {
        w.put(0xC5u32.reverse_bits() >> 24, 8);
        w.put(0, 5);
    }
    w.put(0, 7);
    w.out
}

/* Records of a .vec file: a u32 little-endian length, then that many bytes. */
pub fn records(b: &[u8]) -> Vec<&[u8]> {
    let (mut out, mut i) = (Vec::new(), 0);
    while i + 4 <= b.len() {
        let n = u32::from_le_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]]) as usize;
        out.push(&b[i + 4..i + 4 + n]);
        i += 4 + n;
    }
    out
}
