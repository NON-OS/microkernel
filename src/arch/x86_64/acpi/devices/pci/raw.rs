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

use alloc::vec::Vec;

use super::window::BusWindow;
use crate::arch::x86_64::acpi::parser;

/// (segment, bus, device, function) of every function the MCFG windows
/// answer for.
pub fn enumerate_pci_raw() -> Vec<(u16, u8, u8, u8)> {
    let mut devices = Vec::new();
    for seg in parser::pcie_segments() {
        for bus in seg.start_bus..=seg.end_bus {
            let Some(win) = BusWindow::map(&seg, bus) else {
                continue;
            };
            for device in 0..32u8 {
                if win.read16(device, 0, 0) == 0xFFFF {
                    continue;
                }
                let functions = if win.read8(device, 0, 0x0E) & 0x80 != 0 { 8 } else { 1 };
                for function in 0..functions {
                    if win.read16(device, function, 0) != 0xFFFF {
                        devices.push((seg.segment, bus, device, function));
                    }
                }
            }
        }
    }
    devices
}
