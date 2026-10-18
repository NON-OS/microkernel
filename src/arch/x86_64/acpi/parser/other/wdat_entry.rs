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

//! WDAT, the watchdog action table, from the table's bytes with no pointer
//! read: `wdat_load` keeps what it yields, `kernel_proofs` mounts these
//! files by path. After the 36-byte header: the watchdog header's length
//! u32, the controller's PCI segment u16, bus, device and function u8
//! (0xFF each when it is not on PCI), three reserved bytes, the timer
//! period u32 (milliseconds a count), the maximum and minimum count u32,
//! flags u8 (bit 0 enabled, bit 7 stopped in sleep states), three
//! reserved bytes, the instruction count u32, then the instructions
//! (`wdat_decode`). Recalled from Microsoft's published WDAT
//! description, not read: unverified.

use super::wdat_decode::{instruction, Instruction, ENTRY_LEN};
use alloc::vec::Vec;

const ENTRIES_AT: usize = 68;
/// Instructions kept; a controller needs a few dozen at most.
pub const MAX_ENTRIES: usize = 128;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Wdat {
    /// Segment, bus, device, function.
    pub pci: (u16, u8, u8, u8),
    pub period_ms: u32,
    pub max_count: u32,
    pub min_count: u32,
    pub enabled: bool,
    pub stopped_in_sleep: bool,
    pub entries: Vec<Instruction>,
    /// Entries dropped as malformed: an unknown instruction kind, or a
    /// register outside system memory and I/O.
    pub malformed: u32,
}

fn u32_at(t: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([t[at], t[at + 1], t[at + 2], t[at + 3]])
}

/// None when the table is too short for its watchdog header.
pub fn wdat(table: &[u8]) -> Option<Wdat> {
    let t = table.get(..ENTRIES_AT)?;
    let mut w = Wdat {
        pci: (u16::from_le_bytes([t[40], t[41]]), t[42], t[43], t[44]),
        period_ms: u32_at(t, 48),
        max_count: u32_at(t, 52),
        min_count: u32_at(t, 56),
        enabled: t[60] & 1 != 0,
        stopped_in_sleep: t[60] & 0x80 != 0,
        ..Wdat::default()
    };
    let count = (u32_at(t, 64) as usize).min(MAX_ENTRIES);
    for e in table[ENTRIES_AT..].chunks_exact(ENTRY_LEN).take(count) {
        match instruction(e) {
            Some(i) => w.entries.push(i),
            None => w.malformed += 1,
        }
    }
    Some(w)
}
