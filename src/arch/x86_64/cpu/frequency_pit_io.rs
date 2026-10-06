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

//! Port access for the PIT measurement in frequency_pit.rs.

use core::arch::asm;

#[inline]
pub(super) unsafe fn outb(port: u16, value: u8) {
    // SAFETY: a one-byte write to a PIT port the caller names; on a board
    // whose PIT is gated the write is dropped, and it touches no memory.
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
    }
}

#[inline]
pub(super) unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    // SAFETY: a one-byte read of a PIT port the caller names; a gated PIT
    // reads back all ones, and the read touches no memory.
    unsafe {
        asm!("in al, dx", in("dx") port, out("al") value, options(nomem, nostack, preserves_flags));
    }
    value
}
