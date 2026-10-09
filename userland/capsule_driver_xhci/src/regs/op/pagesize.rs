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

//! PAGESIZE (xHCI 1.2 section 5.4.3): bit n set means the controller uses
//! pages of 2^(n+12) bytes. Scratchpad buffers are that size and aligned to
//! it. Every controller seen so far reports 4 KiB; the register is read so
//! one that does not is either served or refused, never handed 4 KiB pages
//! it would overrun.

use crate::regs::mmio_read32;

const PAGESIZE: u64 = 0x08;
/// The largest page served: the DMA pool hands out at most 16 pages in one
/// grant, and an aligned page is carved out of a grant twice its size.
pub const MAX_PAGE_BYTES: u64 = 32 * 1024;

pub fn pagesize_read(op_base: u64) -> u32 {
    mmio_read32(op_base + PAGESIZE)
}

/// The page size in bytes the register names, the smallest when several
/// bits are set (as Linux takes it), or `None` when no bit is set.
pub fn page_bytes(pagesize: u32) -> Option<u64> {
    let bits = pagesize & 0xFFFF;
    if bits == 0 {
        return None;
    }
    Some(4096u64 << bits.trailing_zeros())
}
