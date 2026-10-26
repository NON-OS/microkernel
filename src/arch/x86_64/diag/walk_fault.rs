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

//! The four page-table entries that translate a faulting user address,
//! read through the directmap. A fault report with only cr2 and the error
//! code says what the CPU saw; the entries say which table and which
//! frame it saw it in.

use crate::memory::layout::DIRECTMAP_BASE;

const PRESENT: u64 = 1 << 0;
const HUGE: u64 = 1 << 7;
const PHYS_MASK: u64 = 0x000F_FFFF_FFFF_F000;

pub(super) fn print_walk(cr3: u64, va: u64) {
    let shifts = [39u32, 30, 21, 12];
    let mut table = cr3 & PHYS_MASK;
    let mut l = crate::sys::serial::Line::new();
    l.str(b"[TRAP WALK]");
    for (level, shift) in shifts.iter().enumerate() {
        let index = (va >> shift) & 0x1FF;
        let slot = (DIRECTMAP_BASE + table + index * 8) as *const u64;
        // SAFETY: eK@nonos.systems - `table` is a page-table frame taken
        // from cr3 or a present entry above it, so the directmap maps it,
        // and `index` is below 512, inside that 4 KiB frame.
        let entry = unsafe { core::ptr::read_volatile(slot) };
        l.str(b" l").str(&[b'4' - level as u8]).str(b"=").hex_lower(entry);
        if entry & PRESENT == 0 || (level > 0 && level < 3 && entry & HUGE != 0) {
            break;
        }
        table = entry & PHYS_MASK;
    }
    // From the trap path, so through the fatal writer like the trap line.
    l.end_fatal();
}
