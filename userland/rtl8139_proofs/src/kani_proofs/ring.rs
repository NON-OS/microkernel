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

use crate::constants::dma::{RX_BUF_BYTES, RX_BUF_DATA_BYTES};
use crate::ring::{copy, u16_at, u8_at};

/* RCR.WRAP is set, so the driver reads linearly into the slack after the
 * ring: the byte at an offset inside the allocation, zero past it. */
fn at(ring: &[u8; RX_BUF_BYTES], off: usize) -> u8 {
    if off < RX_BUF_BYTES {
        ring[off]
    } else {
        0
    }
}

/// For every offset: the byte read never panics, never leaves the allocation,
/// and returns the byte there, or zero past the end.
#[kani::proof]
fn byte_reads_are_total_and_confined() {
    let ring = [0u8; RX_BUF_BYTES];
    let off: usize = kani::any();
    assert!(u8_at(ring.as_ptr() as u64, off) == at(&ring, off));
}

/// For every header offset the walk can hold: both bytes of a u16 read come
/// from the allocation, little-endian.
#[kani::proof]
fn u16_reads_are_total_and_confined() {
    let ring = [0u8; RX_BUF_BYTES];
    let off: usize = kani::any();
    kani::assume(off < 4 * RX_BUF_DATA_BYTES);
    let v = u16_at(ring.as_ptr() as u64, off);
    assert!(v == at(&ring, off) as u16 | ((at(&ring, off + 1) as u16) << 8));
}

/// For every start and hostile length: the copy stays inside the allocation
/// and the caller's buffer, and fills exactly min(len, out.len()) bytes.
#[kani::proof]
#[kani::unwind(10)]
fn the_copy_is_confined_to_allocation_and_buffer() {
    let ring = [0u8; RX_BUF_BYTES];
    let mut out = [0xEEu8; 8];
    let start: usize = kani::any();
    kani::assume(start < 4 * RX_BUF_DATA_BYTES);
    let len: usize = kani::any();
    copy(ring.as_ptr() as u64, start, &mut out, len);
    let filled = if len < out.len() { len } else { out.len() };
    let mut i = 0;
    while i < out.len() {
        assert!(out[i] == if i < filled { at(&ring, start + i) } else { 0xEE });
        i += 1;
    }
}
