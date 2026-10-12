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
//! Which class driver holds which interface of a device, so two may share
//! one: a composite key's keyboard is usb_hid0's while its FIDO interface
//! is driver.fido0's. The slot is the device's; a driver gives back only
//! its own interfaces, and the slot goes with the last of them. A device
//! no driver claimed an interface of is not shared, as before.

use alloc::vec::Vec;

/// Interfaces held at once, over every device.
const CLAIMS: usize = 64;
pub const E_BUSY: i32 = -16;
pub const E_INVAL: i32 = -22;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) struct Claim {
    pub(super) slot: u8,
    pub(super) port: u8,
    pub(super) interface: u8,
    pub(super) pid: u32,
}

#[derive(Default)]
pub struct Claims {
    pub(super) list: Vec<Claim>,
}

impl Claims {
    pub fn claim(&mut self, slot: u8, port: u8, interface: u8, pid: u32) -> Result<(), i32> {
        if slot == 0 || self.list.iter().any(|c| c.slot == slot && c.port != port) {
            return Err(E_INVAL);
        }
        match self.list.iter().find(|c| c.slot == slot && c.interface == interface) {
            Some(c) if c.pid == pid => Ok(()),
            Some(_) => Err(E_BUSY),
            None if self.list.len() >= CLAIMS => Err(E_BUSY),
            None => {
                self.list.push(Claim { slot, port, interface, pid });
                Ok(())
            }
        }
    }

    /// The shared slot on `port` and its claimed interfaces (0 to 31).
    pub fn on_port(&self, port: u8) -> Option<(u8, u32)> {
        let slot = self.list.iter().find(|c| c.port == port)?.slot;
        let held = self.list.iter().filter(|c| c.slot == slot && c.interface < 32);
        Some((slot, held.fold(0, |m, c| m | 1 << c.interface)))
    }
}
