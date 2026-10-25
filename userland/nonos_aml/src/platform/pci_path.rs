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

//! The bus, device and function a PCI_Config region reaches: the host
//! bridge's segment and bus, then down each bridge on the path by its
//! secondary bus number (PCI-to-PCI Bridge Architecture 1.2, 3.2.5.3,
//! config offset 0x19). Each _ADR is device in the high word and function
//! in the low (ACPI 6.5, 6.1.1).

use crate::exec::PciPath;

/// The configuration register holding a bridge's secondary bus number.
pub const SECONDARY_BUS: u16 = 0x19;

/// `secondary(seg, bus, dev, func)` reads a bridge's secondary bus. None
/// when the path is empty, an _ADR is out of range, or a read fails.
pub fn function_of(
    path: &PciPath,
    secondary: &mut dyn FnMut((u16, u8, u8, u8)) -> Option<u8>,
) -> Option<(u16, u8, u8, u8)> {
    let (last, bridges) = path.adrs.split_last()?;
    let split = |adr: u32| -> Option<(u8, u8)> {
        let (dev, func) = (adr >> 16, adr & 0xFFFF);
        (dev < 32 && func < 8).then_some((dev as u8, func as u8))
    };
    let mut bus = path.bus;
    for adr in bridges {
        let (dev, func) = split(*adr)?;
        bus = secondary((path.segment, bus, dev, func))?;
    }
    let (dev, func) = split(*last)?;
    Some((path.segment, bus, dev, func))
}
