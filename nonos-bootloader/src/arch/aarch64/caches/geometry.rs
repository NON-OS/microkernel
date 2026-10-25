// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! The arithmetic of cache maintenance, pure so it is held on the host
//! (loader_arm_proofs). From the Arm Architecture Reference Manual: CTR_EL0
//! gives the smallest data-cache line, and DC CIVAC works one line at a
//! time. The walk by set and way is assembly, `asm/walk.S`.

/// The smallest data-cache line in bytes: CTR_EL0.DminLine, bits 19:16, is
/// log2 of the line in 4-byte words.
pub fn dminline_bytes(ctr: u64) -> u64 {
    4 << ((ctr >> 16) & 0xF)
}

/// The line-aligned addresses that cover `[start, start + len)`, as
/// (first, count). None when the range wraps.
pub fn lines(start: u64, len: u64, line: u64) -> Option<(u64, u64)> {
    if len == 0 {
        return Some((start & !(line - 1), 0));
    }
    let end = start.checked_add(len)?;
    let first = start & !(line - 1);
    let last = end.checked_add(line - 1)? & !(line - 1);
    Some((first, (last - first) / line))
}
