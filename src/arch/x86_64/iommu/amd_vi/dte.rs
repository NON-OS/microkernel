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
//! four quadwords: V, TV, the paging mode and root, IR and IW, and the
//! domain id, each where the spec's entry format puts it. Pure.

pub type Dte = [u64; 4];

const VALID: u64 = 1 << 0;
const TRANSLATION_VALID: u64 = 1 << 1;
const READ: u64 = 1 << 61;
const WRITE: u64 = 1 << 62;
const ROOT_MASK: u64 = 0x000F_FFFF_FFFF_F000;

/// No DMA at all: V and TV with Mode 0, neither IR nor IW. Its interrupts
/// are refused too (IV set, no table, K4) until a bind gives it a table.
pub const fn blocked() -> Dte {
    [VALID | TRANSLATION_VALID, 0, super::intr_rule::ABORT_ALL, 0]
}

/// DMA reaches physical addresses untranslated (Mode 0 with IR and IW). The
/// kernel writes it for no device; the identity count judges by it.
pub const fn passthrough() -> Dte {
    [VALID | TRANSLATION_VALID | READ | WRITE, 0, 0, 0]
}

/// `entry`'s DMA half with `from`'s interrupt quadword: moving a device
/// between domains never touches how its interrupts are remapped.
pub const fn keep_interrupts(entry: Dte, from: Dte) -> Dte {
    [entry[0], entry[1], from[2], entry[3]]
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

/// A writer entry with no valid translation, so the device could write
/// physical memory untranslated: never the kernel's own, refused before enable.
pub const fn stray_writer(entry: Dte) -> bool {
    allows_write(entry) && entry[0] & TRANSLATION_VALID == 0
}
