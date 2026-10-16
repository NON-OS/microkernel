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

//! Reading a function's configuration space for the hot-plug ports: its
//! PCI Express capability, and the bridges on the buses below a bridge
//! (PCI Local Bus: the capability pointer at 0x34, the header type's
//! multi-function bit; recalled, unverified).

use super::find::Bdf;
use super::regs::CAP_ID;
use crate::dpower::bridge_bus;
use crate::sys::acpi_pci::pci_read;
use alloc::vec::Vec;

const MAX_BRIDGES: usize = 64;
const CAP_LIST: u16 = 0x34;

/// All ones for a read that fails, as a function that is not there reads.
pub fn read(f: Bdf, at: u16, w: u8) -> u32 {
    pci_read(f, at, w).unwrap_or(u32::MAX)
}

/// The PCI Express capability, by walking the capability list.
pub fn pcie_cap(f: Bdf) -> Option<u16> {
    find_cap(f, CAP_ID)
}

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

pub fn bridges_below(seen: &mut Vec<Bdf>) {
    let mut i = 0;
    while i < seen.len() && seen.len() < MAX_BRIDGES {
        if let Some(bus) = bridge_bus(seen[i]) {
            for d in 0..32u8 {
                let f0 = (seen[i].0, bus, d, 0);
                if read(f0, 0, 2) == 0xFFFF {
                    continue;
                }
                let funcs = if read(f0, 0x0E, 1) & 0x80 != 0 { 8 } else { 1 };
                for g in (0..funcs).map(|f| (f0.0, bus, d, f)) {
                    if bridge_bus(g).is_some() && !seen.contains(&g) {
                        seen.push(g);
                    }
                }
            }
        }
        i += 1;
    }
}
