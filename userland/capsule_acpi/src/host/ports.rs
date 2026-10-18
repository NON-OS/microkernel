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

//! SystemIO: the region asked for whole on first touch, falling back to the
//! ports the access needs; every access is the kernel's `in` or `out`.

use crate::sys::acpi::{port_read, port_write, ports};
use alloc::vec::Vec;

#[derive(Default)]
pub struct Ports {
    held: Vec<(u64, u64)>,
}

impl Ports {
    fn holds(&self, port: u64, end: u64) -> bool {
        self.held.iter().any(|(b, e)| *b <= port && end <= *e)
    }

    fn ask(&mut self, base: u64, len: u64) -> bool {
        let ok = ports(base, len).is_ok();
        if ok {
            self.held.push((base, base + len));
        }
        ok
    }

    pub fn ensure(&mut self, port: u64, width: u64, region: (u64, u64)) -> bool {
        let Some(end) = port.checked_add(width) else { return false };
        self.holds(port, end)
            || ((self.ask(region.0, region.1) && self.holds(port, end)) || self.ask(port, width))
    }

    /// Port accesses are 1, 2 or 4 bytes; an eight-byte field is two.
    pub fn read(&mut self, port: u64, width: u32) -> Option<u64> {
        match width {
            1 | 2 | 4 => port_read(port, width as u8).ok().map(u64::from),
            8 => {
                let lo = port_read(port, 4).ok()? as u64;
                Some(lo | (port_read(port + 4, 4).ok()? as u64) << 32)
            }
            _ => None,
        }
    }

    pub fn write(&mut self, port: u64, width: u32, v: u64) -> bool {
        match width {
            1 | 2 | 4 => port_write(port, width as u8, v as u32).is_ok(),
            8 => {
                port_write(port, 4, v as u32).is_ok()
                    && port_write(port + 4, 4, (v >> 32) as u32).is_ok()
            }
            _ => false,
        }
    }
}
