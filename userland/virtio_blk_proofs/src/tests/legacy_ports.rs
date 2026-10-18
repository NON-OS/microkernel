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

//! The legacy device's registers as the driver's port reads and writes
//! reach them.

use nonos_libc::Ports;

use super::bench::Attached;

/// The configuration starts here, or four bytes on with MSI-X on.
const CONFIG: u16 = 0x14;

impl Ports for Attached {
    fn read(&mut self, port: u16, _width: u8) -> u32 {
        let mut d = self.0.borrow_mut();
        let config = if d.msix { CONFIG + 4 } else { CONFIG };
        match port {
            0x00 => d.host_features,
            0x04 => d.guest_features,
            0x08 => d.pfn,
            0x0C => u32::from(if d.sel == 0 { d.qnum } else { 0 }),
            0x12 => match d.resetting {
                Some(0) => {
                    d.stopped();
                    d.seen_stopped = true;
                    0
                }
                Some(n) => {
                    d.resetting = Some(if n == u32::MAX { n } else { n - 1 });
                    u32::from(d.status)
                }
                None => {
                    d.seen_stopped |= d.status == 0;
                    u32::from(d.status)
                }
            },
            0x14 | 0x16 if d.msix => u32::from(d.vectors[usize::from(port - 0x14) / 2]),
            p if p == config => d.capacity as u32,
            p if p == config + 4 => (d.capacity >> 32) as u32,
            _ => 0,
        }
    }

    fn write(&mut self, port: u16, _width: u8, value: u32) {
        let mut d = self.0.borrow_mut();
        match port {
            0x04 => d.guest_features = value,
            0x08 => d.pfn = value,
            0x0E => d.sel = value as u16,
            0x10 => d.notify(),
            0x12 if d.resetting.is_some() => d.written_mid_reset |= value != 0,
            0x12 if value == 0 => d.reset_asked(),
            0x12 => d.status = value as u8,
            0x14 | 0x16 if d.msix => d.vectors[usize::from(port - 0x14) / 2] = value as u16,
            _ => {}
        }
    }
}
