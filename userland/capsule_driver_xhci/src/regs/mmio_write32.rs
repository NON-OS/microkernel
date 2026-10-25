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
/// Write the 32-bit register at `addr`, through libc's accessor so every
/// earlier store to DMA memory is seen first.
///
/// # Safety
/// As for `mmio_read32`.
pub(crate) unsafe fn mmio_write32(addr: u64, value: u32) {
    // SAFETY: the caller's contract; libc's access is ordered against
    // DMA memory, so a doorbell follows the TRBs it announces.
    unsafe { nonos_libc::mmio_write32(addr as usize, value) };
}
