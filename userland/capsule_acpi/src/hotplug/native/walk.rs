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

//! Reading a function's configuration space through MkAcpiPci, and its
//! capability list (PCI Local Bus: the capability pointer at 0x34;
//! recalled, unverified), for device power. The hot-plug ports never come
//! here: MkAcpiSlot answers for them.

use super::find::Bdf;
use crate::sys::acpi_pci::pci_read;

const CAP_LIST: u16 = 0x34;

/// All ones for a read that fails, as a function that is not there reads.
pub fn read(f: Bdf, at: u16, w: u8) -> u32 {
    pci_read(f, at, w).unwrap_or(u32::MAX)
}

/// The PCI Express capability, by walking the capability list.
/// The capability `id`, by walking the capability list.
pub fn find_cap(f: Bdf, id: u8) -> Option<u16> {
    let mut at = read(f, CAP_LIST, 1) as u16 & 0xFC;
    for _ in 0..48 {
        if !(0x40..=0xFC).contains(&at) {
            return None;
        }
        if read(f, at, 1) as u8 == id {
            return Some(at);
        }
        at = read(f, at + 1, 1) as u16 & 0xFC;
    }
    None
}
