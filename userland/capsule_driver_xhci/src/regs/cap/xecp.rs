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

//! The extended capability list (xHCI 1.2 section 7). HCCPARAMS1[31:16] is
//! the first capability's offset in 32-bit words from the register base;
//! each capability's bits 15:8 are the offset of the next in 32-bit words
//! from itself, and zero ends the list. On silicon the list sits far past
//! the operational registers (Intel PCH controllers start it near 0x8000),
//! so every read is checked against the mapped window first: a pointer that
//! leads out of the window ends the walk instead of faulting the capsule.

use crate::constants::HCCPARAMS1;
use crate::regs::mmio_read32;

/// The longest list walked, so a list whose pointers loop still ends.
const XECP_WALK_LIMIT: usize = 256;

/// Visit each capability in list order with its offset from the register
/// base and its first dword, reading through `read` (an offset from the
/// base). A capability is visited only when its first `need` bytes lie in
/// the `mapped_len` window, and the walk stops at the first that does not.
/// `visit` returns false to stop early.
pub fn walk_ext_caps(
    hccparams1: u32,
    mapped_len: u64,
    need: u64,
    read: impl Fn(u64) -> u32,
    mut visit: impl FnMut(u64, u32) -> bool,
) {
    let mut offset = ((hccparams1 >> 16) & 0xFFFF) as u64 * 4;
    if offset == 0 {
        return;
    }
    for _ in 0..XECP_WALK_LIMIT {
        let end = match offset.checked_add(need.max(4)) {
            Some(end) => end,
            None => return,
        };
        if end > mapped_len {
            return;
        }
        let dw0 = read(offset);
        // A function that dropped off the bus reads all ones.
        if dw0 == u32::MAX || !visit(offset, dw0) {
            return;
        }
        let next = ((dw0 >> 8) & 0xFF) as u64;
        if next == 0 {
            return;
        }
        offset += next * 4;
    }
}

/// The same walk over the mapped registers at `mmio_base`.
pub fn ext_caps(mmio_base: u64, mapped_len: u64, need: u64, visit: impl FnMut(u64, u32) -> bool) {
    let hccparams1 = mmio_read32(mmio_base + HCCPARAMS1);
    walk_ext_caps(hccparams1, mapped_len, need, |off| mmio_read32(mmio_base + off), visit);
}
