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

//! The cache lines a DMA sync flushes, free of the instruction so the host
//! proofs check it. Every x86-64 CPU flushes at least 64 bytes per CLFLUSH
//! (CPUID.01H:EBX bits 15:8 give 8 or more), so stepping by 64 from the line
//! that holds the first byte never skips one.

pub const DMA_SYNC_LINE: u64 = 64;

/// The first line address and the number of lines `[addr, addr + len)`
/// touches; none for an empty range or one that wraps.
pub const fn dma_sync_lines(addr: u64, len: u64) -> (u64, u64) {
    let end = match addr.checked_add(len) {
        Some(end) if len != 0 => end,
        _ => return (addr, 0),
    };
    let first = addr & !(DMA_SYNC_LINE - 1);
    (first, (end - first).div_ceil(DMA_SYNC_LINE))
}
