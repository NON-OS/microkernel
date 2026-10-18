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

//! Frames for the parse proofs: a seeded generator, a frame built from its
//! fields, and the little-endian reads a reply is checked with.

use crate::protocol::{HDR_LEN, MAGIC, VERSION};

pub(crate) const FUZZ_ROUNDS: usize = 250_000;

pub(crate) fn xorshift(s: &mut u64) -> u64 {
    let mut x = *s;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *s = x;
    x
}

pub(crate) fn frame(
    op: u16,
    flags: u16,
    request_id: u32,
    payload_len: u32,
    body: &[u8],
) -> Vec<u8> {
    let mut f = Vec::with_capacity(HDR_LEN + body.len());
    f.extend_from_slice(&MAGIC.to_le_bytes());
    f.extend_from_slice(&VERSION.to_le_bytes());
    f.extend_from_slice(&op.to_le_bytes());
    f.extend_from_slice(&flags.to_le_bytes());
    f.extend_from_slice(&[0, 0]);
    f.extend_from_slice(&request_id.to_le_bytes());
    f.extend_from_slice(&payload_len.to_le_bytes());
    f.extend_from_slice(body);
    f
}

pub(crate) fn le16(b: &[u8], off: usize) -> u16 {
    u16::from_le_bytes([b[off], b[off + 1]])
}

pub(crate) fn le32(b: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([b[off], b[off + 1], b[off + 2], b[off + 3]])
}
