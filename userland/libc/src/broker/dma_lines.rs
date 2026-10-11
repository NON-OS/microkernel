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
//! that holds the first byte never skips one. An aarch64 CPU states its
//! smallest data line in CTR_EL0.DminLine (Arm ARM, DDI 0487, CTR_EL0), and
//! stepping by that never skips one either.

pub const DMA_SYNC_LINE: u64 = 64;

/// The first line address and the number of lines `[addr, addr + len)`
/// touches; none for an empty range or one that wraps.
pub const fn dma_sync_lines(addr: u64, len: u64) -> (u64, u64) {
    dma_sync_lines_of(addr, len, DMA_SYNC_LINE)
}

/// As `dma_sync_lines`, for lines of `line` bytes, a power of two.
pub const fn dma_sync_lines_of(addr: u64, len: u64, line: u64) -> (u64, u64) {
    let end = match addr.checked_add(len) {
        Some(end) if len != 0 => end,
        _ => return (addr, 0),
    };
    let first = addr & !(line - 1);
    (first, (end - first).div_ceil(line))
}

/// The smallest data cache line, in bytes, a CTR_EL0 value states:
/// DminLine, bits 19:16, is the log2 of the number of 4-byte words.
pub const fn ctr_dminline_bytes(ctr: u64) -> u64 {
    4 << ((ctr >> 16) & 0xf)
}
