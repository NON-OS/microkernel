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

//! Which devices a client may address. driver.i2c_hid0 parses descriptors
//! and reports the device controls, so if it is ever subverted it must find
//! the touchpad on the end of OP_TRANSFER and nothing else: not the PMIC,
//! the USB-PD controller, the fuel gauge, an SPD EEPROM or an I2C TPM that
//! share the bus. The reach is set at the bind, holds as many addresses as
//! setup keeps firmware candidates, each with the HID descriptor register
//! its declaration names, and after that only narrows (narrow.rs).

use crate::discover::MAX_TARGETS;

#[derive(Clone, Copy, Default)]
pub struct Reach {
    addrs: [u8; MAX_TARGETS],
    /// The HID descriptor register each address's declaration names.
    regs: [u16; MAX_TARGETS],
    len: usize,
}

impl Reach {
    /// The 7-bit addresses among `declared`, each with the HID descriptor
    /// register its declaration names: the first MAX_TARGETS distinct ones;
    /// zero (the general call) and anything wider than seven bits never
    /// count.
    pub fn declared(declared: impl IntoIterator<Item = (u8, u16)>) -> Self {
        let mut r = Reach::default();
        for (a, reg) in declared {
            if r.len < MAX_TARGETS && (1..=0x7F).contains(&a) && !r.allows(a) {
                (r.addrs[r.len], r.regs[r.len]) = (a, reg);
                r.len += 1;
            }
        }
        r
    }

    pub fn allows(&self, addr: u8) -> bool {
        addr != 0 && self.addrs().contains(&addr)
    }

    /// The HID descriptor register declared for `addr`, when it is in reach.
    pub fn desc_reg(&self, addr: u8) -> Option<u16> {
        let i = self.addrs().iter().position(|&a| a != 0 && a == addr)?;
        self.regs.get(i).copied()
    }

    pub(super) fn addrs(&self) -> &[u8] {
        self.addrs.get(..self.len).unwrap_or(&[])
    }
}
