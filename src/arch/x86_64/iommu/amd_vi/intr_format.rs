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

//! Remapped interrupts on AMD-Vi (48882, 2.2.5; the field positions are
//! recalled, unverified): the interrupt quadword of a device table entry,
//! and the 32-bit remapping table entry. Pure.
//!
//! Each remapped device has a table of its own, and the index its MSI
//! carries (data bits 10:0) is looked up only there. Left unremapped, a
//! device that can write memory can send any MSI: any vector on any CPU.
//! Remapped, it raises only the entries the broker wrote for it; INIT,
//! ExtINT, NMI and the LINT pins pass from none of them.

use super::dte::Dte;

/// A device's table holds 2^`LEN_LOG2` entries of 4 bytes.
pub const LEN_LOG2: u8 = 8;
pub const ENTRIES: usize = 1 << LEN_LOG2;

/// Quadword 2 of the entry: IV (bit 128), IntTabLen (132:129), the table
/// root (179:134, 128-byte aligned) and IntCtl (189:188, 10b remapped).
const IV: u64 = 1 << 0;
const LEN_SHIFT: u32 = 1;
const ROOT_MASK: u64 = 0x000F_FFFF_FFFF_FFC0;
const INT_CTL_REMAPPED: u64 = 0b10 << 60;

/// The interrupt quadword for a device remapped through `table_phys`. Every
/// pass bit stays clear.
pub const fn remapped_quadword(table_phys: u64) -> u64 {
    IV | ((LEN_LOG2 as u64) << LEN_SHIFT) | (table_phys & ROOT_MASK) | INT_CTL_REMAPPED
}

pub const fn remaps(entry: Dte) -> bool {
    entry[2] & IV != 0
}

/// The DMA half of one entry with another's interrupt quadword, so a move
/// between domains never turns a device's remapping off (dte.rs).
pub use super::dte::keep_interrupts;

/// Whether two entries give a device the same DMA, whatever its interrupts.
pub const fn same_dma(a: Dte, b: Dte) -> bool {
    a[0] == b[0] && a[1] == b[1] && a[3] == b[3]
}

/// RemapEn, IntType 000 (fixed), RqEoi, DM and GuestMode clear (physical
/// destination), the destination in bits 15:8 and the vector in 23:16.
pub const fn irte(vector: u8, dest: u8) -> u32 {
    1 | ((dest as u32) << 8) | ((vector as u32) << 16)
}

/// The MSI a remapped device sends: the usual address, the index as data.
pub const MSI_ADDRESS: u64 = 0xFEE0_0000;

pub const fn msi_data(index: u16) -> u32 {
    index as u32 & 0x7FF
}
