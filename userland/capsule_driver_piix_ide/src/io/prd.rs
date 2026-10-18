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

//! The physical region descriptor table for one transfer: the buffer's
//! first `len` bytes cut at every 64 KiB boundary, which no region may
//! cross, the last descriptor marked end of table. Pure, for the host
//! proofs.

use crate::constants::bmide::{PRD_BOUNDARY, PRD_BYTES, PRD_EOT};

/// Write the table for `[addr, addr + len)` into `out` and return how many
/// descriptors it holds; None when `len` is zero, the range does not fit in
/// 32 bits, or `out` is too short.
pub fn build(addr: u64, len: usize, out: &mut [u8]) -> Option<usize> {
    let end = addr.checked_add(len as u64)?;
    if len == 0 || end > 1 << 32 {
        return None;
    }
    let mut at = addr;
    let mut n = 0usize;
    while at < end {
        let next = ((at / PRD_BOUNDARY) + 1) * PRD_BOUNDARY;
        let stop = next.min(end);
        let bytes = stop - at;
        let slot = out.get_mut(n * PRD_BYTES..(n + 1) * PRD_BYTES)?;
        let flags = if stop == end { PRD_EOT } else { 0 };
        slot[0..4].copy_from_slice(&(at as u32).to_le_bytes());
        // A full 64 KiB region is written as a count of zero.
        slot[4..6].copy_from_slice(&(bytes as u16).to_le_bytes());
        slot[6..8].copy_from_slice(&flags.to_le_bytes());
        n += 1;
        at = stop;
    }
    Some(n)
}
