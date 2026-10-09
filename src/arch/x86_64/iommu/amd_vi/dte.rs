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

//! Device table entries (AMD IOMMU spec 48882, section 2.2.2.1), 256 bits as
//! four quadwords, with the bits Linux sets in amd/iommu.c (set_dte_entry)
//! and amd/init.c (init_device_table_dma). Pure.

pub type Dte = [u64; 4];

const VALID: u64 = 1 << 0;
const TRANSLATION_VALID: u64 = 1 << 1;
const READ: u64 = 1 << 61;
const WRITE: u64 = 1 << 62;
const ROOT_MASK: u64 = 0x000F_FFFF_FFFF_F000;

/// No DMA at all. V and TV set with Mode 0 and neither IR nor IW: the unit
/// translates nothing and permits nothing. Interrupt fields stay zero (IV
/// clear), so the device's interrupts pass as they do today.
pub const fn blocked() -> Dte {
    [VALID | TRANSLATION_VALID, 0, 0, 0]
}

/// DMA reaches physical addresses untranslated: Mode 0 with IR and IW,
/// what Linux programs for an identity domain.
pub const fn passthrough() -> Dte {
    [VALID | TRANSLATION_VALID | READ | WRITE, 0, 0, 0]
}

/// DMA translated through `levels` of page table from `root`, tagged with
/// `domain` in the second quadword's low sixteen bits.
pub const fn translated(root: u64, levels: u8, domain: u16) -> Dte {
    let mode = ((levels & 0x7) as u64) << 9;
    [VALID | TRANSLATION_VALID | mode | (root & ROOT_MASK) | READ | WRITE, domain as u64, 0, 0]
}

pub const fn domain_of(entry: Dte) -> u16 {
    entry[1] as u16
}

pub const fn mode_of(entry: Dte) -> u8 {
    ((entry[0] >> 9) & 0x7) as u8
}

pub const fn root_of(entry: Dte) -> u64 {
    entry[0] & ROOT_MASK
}

/// Whether the entry lets the device write memory at all.
pub const fn allows_write(entry: Dte) -> bool {
    entry[0] & (VALID | WRITE) == VALID | WRITE
}

/// A writer entry with no valid translation: the device could write untranslated
/// physical memory, which is exactly what the unit must never let through. The
/// kernel's own entries are either blocked (no write) or translated (a page
/// table), so any such entry is a stray one the pre-enable scan refuses.
pub const fn stray_writer(entry: Dte) -> bool {
    allows_write(entry) && entry[0] & TRANSLATION_VALID == 0
}
