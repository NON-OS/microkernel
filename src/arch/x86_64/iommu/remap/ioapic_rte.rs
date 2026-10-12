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

//! An I/O APIC redirection entry in remappable format (VT-d 3.4, 5.1.5.1;
//! bit positions recalled, unverified): the entry names a remapping table
//! index instead of a CPU, so the unit checks the I/O APIC's source id and
//! delivers what the table says. With CFI cleared, a compatibility format
//! entry is blocked, so every line must be in this form first (L44). Pure.

/// Interrupt Format, bit 48: the entry is remappable.
const REMAPPABLE: u64 = 1 << 48;
const POLARITY_LOW: u64 = 1 << 13;
const TRIGGER_LEVEL: u64 = 1 << 15;
const MASKED: u64 = 1 << 16;

/// How the line is driven, as the redirection entry carries it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Line {
    pub level: bool,
    pub active_low: bool,
    pub masked: bool,
}

/// The entry raising table entry `index`. Index bits 14:0 sit in bits 63:49
/// and bit 15 in bit 11; the vector field carries the entry's vector, which
/// a level line's EOI is matched by.
pub const fn remappable_rte(index: u16, vector: u8, line: Line) -> u64 {
    let low_index = ((index as u64) & 0x7FFF) << 49;
    let high_index = (((index as u64) >> 15) & 1) << 11;
    let polarity = if line.active_low { POLARITY_LOW } else { 0 };
    let trigger = if line.level { TRIGGER_LEVEL } else { 0 };
    let mask = if line.masked { MASKED } else { 0 };
    REMAPPABLE | low_index | high_index | polarity | trigger | mask | vector as u64
}

/// The table index a remappable entry names, `None` for a compatibility
/// format one.
pub const fn rte_index(entry: u64) -> Option<u16> {
    if entry & REMAPPABLE == 0 {
        return None;
    }
    Some((((entry >> 49) & 0x7FFF) | (((entry >> 11) & 1) << 15)) as u16)
}
