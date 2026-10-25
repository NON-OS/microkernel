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
/// Read the 32-bit register at `addr`, through libc's accessor so the
/// load is ordered before any later read of DMA memory it vouches for.
///
/// # Safety
/// `addr` is a 4-byte aligned register inside a register window this
/// process mapped (`mk_mmio_map`) and still holds.
pub(crate) unsafe fn mmio_read32(addr: u64) -> u32 {
    // SAFETY: the caller's contract; libc's access is ordered against
    // DMA memory, so a doorbell follows the TRBs it announces.
    unsafe { nonos_libc::mmio_read32(addr as usize) }
}
