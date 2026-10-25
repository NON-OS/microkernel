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

//! The generator, reader and frame builders the request proofs share.

use alloc::vec::Vec;

pub(super) const MAGIC: u32 = 0x4E4D_4B54;

pub(super) fn xorshift(s: &mut u64) -> u64 {
    let mut x = *s;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *s = x;
    x
}

pub(super) fn le32(b: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([b[off], b[off + 1], b[off + 2], b[off + 3]])
}

pub(super) fn frame(op: u16, request_id: u32, payload_len: u32) -> Vec<u8> {
    let mut f = Vec::new();
    f.extend_from_slice(&MAGIC.to_le_bytes());
    f.extend_from_slice(&1u16.to_le_bytes());
    f.extend_from_slice(&op.to_le_bytes());
    f.extend_from_slice(&[0, 0, 0, 0]);
    f.extend_from_slice(&request_id.to_le_bytes());
    f.extend_from_slice(&payload_len.to_le_bytes());
    f
}

/// A length prefix: right, one over, one under, or anything.
pub(super) fn lp(s: &mut u64, body: &[u8]) -> Vec<u8> {
    let n = body.len() as u32;
    let claimed = match xorshift(s) % 8 {
        0 => n + 1,
        1 => n.saturating_sub(1),
        2 => u32::MAX - (xorshift(s) % 4) as u32,
        3 => xorshift(s) as u32,
        _ => n,
    };
    let mut out = claimed.to_le_bytes().to_vec();
    out.extend_from_slice(body);
    out
}
