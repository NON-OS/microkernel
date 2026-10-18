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
use super::mmio_write32::mmio_write32;
/// Write the 64-bit register at `addr` as two dword stores, low then high
/// (xHCI 1.2 section 5.1, Linux `xhci_write_64`): some controllers
/// mishandle a qword access, and one without 64-bit addressing must be
/// able to take the high dword on its own.
///
/// # Safety
/// `addr` is an 8-byte aligned register inside a register window this
/// process mapped and still holds.
pub(crate) unsafe fn mmio_write64(addr: u64, value: u64) {
    // SAFETY: the caller's contract covers both dwords.
    unsafe {
        mmio_write32(addr, value as u32);
        mmio_write32(addr + 4, (value >> 32) as u32);
    }
}
