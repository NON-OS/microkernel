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

//! The Slot Information config byte and the BAR it names.

/// PCI config offset 0x40: Slot Information (SDHCI 3.0, 2.3). Bits 2:0 name
/// the BAR of the first slot, bits 6:4 the slot count less one.
pub const PCI_SLOT_INFO: u32 = 0x40;

/// The BAR holding slot 0's registers, from the Slot Information byte. A
/// value past the six BARs reads as BAR0, where every known host has it.
pub const fn first_bar(slot_info: u8) -> u8 {
    let bar = slot_info & 0x7;
    if bar > 5 {
        0
    } else {
        bar
    }
}
