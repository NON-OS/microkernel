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

//! Frames as either end builds them, and the test's own little-endian
//! reads, so the models below never lean on the crate they judge.

use super::rng::Rng;
use crate::header::{Header, HDR_LEN, MAGIC, VERSION};

pub fn le16(b: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_le_bytes([*b.get(at)?, *b.get(at + 1)?]))
}

pub fn le32(b: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes([*b.get(at)?, *b.get(at + 1)?, *b.get(at + 2)?, *b.get(at + 3)?]))
}

/// The header fields as the test reads them; None for a wrong magic or
/// version, or a frame too short to hold one.
pub fn header(b: &[u8]) -> Option<Header> {
    if le32(b, 0)? != MAGIC || le16(b, 4)? != VERSION {
        return None;
    }
    let (op, flags, output) = (le16(b, 6)?, le16(b, 8)?, le16(b, 10)?);
    Some(Header { op, flags, output, request_id: le32(b, 12)?, payload_len: le32(b, 16)? })
}

pub fn random_header(r: &mut Rng) -> Header {
    let op = [0x01, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17][r.below(9) as usize];
    Header {
        op,
        flags: r.below(4) as u16,
        output: r.below(3) as u16,
        request_id: r.word(),
        payload_len: 0,
    }
}

/// `body` after a header saying `said` bytes follow, then a lie or two:
/// a wrong magic or version, the frame cut or run on.
pub fn frame(r: &mut Rng, h: Header, said: u32, body: &[u8]) -> Vec<u8> {
    let mut f = vec![0u8; HDR_LEN];
    Header { payload_len: said, ..h }.encode(&mut f).unwrap();
    f.extend_from_slice(body);
    match r.below(10) {
        0 => f[r.below(6) as usize] ^= 1 << r.below(8),
        1 => f.truncate(r.below(f.len() as u32 + 1) as usize),
        2 => f.extend((0..r.below(9)).map(|_| r.next() as u8)),
        _ => {}
    }
    f
}

pub fn bytes(r: &mut Rng, n: usize) -> Vec<u8> {
    let words: Vec<u32> = (0..n.div_ceil(4)).map(|_| r.word()).collect();
    let mut b: Vec<u8> = words.iter().flat_map(|w| w.to_le_bytes()).collect();
    b.truncate(n);
    b
}
