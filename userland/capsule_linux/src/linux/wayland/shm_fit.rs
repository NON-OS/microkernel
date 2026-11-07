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

//! Whether a wl_shm buffer's rectangle lies in its pool, asked as it is
//! made and again each time its pixels are read. Pure, so the host proofs
//! hold it.
//!
//! The read is where it counts: a pool's size can change and its mapping
//! can go between the two, and pixels read at a stride that is not the
//! buffer's own show every row sheared from the one above.

/// Bytes per pixel in both formats this capsule advertises.
pub const BPP: u64 = 4;
/// The widest and tallest buffer taken: twice an 8K display's side. Within
/// MAX_FRAME a buffer one pixel wide could be four million tall, and the
/// window framed around it was sized from that.
pub const MAX_SIDE: u64 = 16_384;

/// Whether the rectangle lies wholly inside a pool of `size` bytes, its rows
/// at least `width` pixels apart, and no side longer than MAX_SIDE.
pub fn fits(offset: u64, width: u64, height: u64, stride: u64, size: u64) -> bool {
    if width == 0 || height == 0 || width > MAX_SIDE || height > MAX_SIDE {
        return false;
    }
    let Some(row) = width.checked_mul(BPP) else {
        return false;
    };
    if stride < row {
        return false;
    }
    let Some(span) = stride.checked_mul(height) else {
        return false;
    };
    matches!(offset.checked_add(span), Some(end) if end <= size)
}

/// The bytes a read of the buffer takes, stride times height, when it fits
/// a pool of `size` bytes; None, and nothing read, when it does not.
pub fn read_len(offset: u64, width: u32, height: u32, stride: u32, size: u64) -> Option<u64> {
    let (w, h, s) = (u64::from(width), u64::from(height), u64::from(stride));
    fits(offset, w, h, s, size).then(|| s * h)
}
