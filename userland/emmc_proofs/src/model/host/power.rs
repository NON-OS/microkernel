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

//! Power Control writes: SD Bus Power on and off, and the voltage select
//! held to what the host offers.

use super::sim::Sim;

impl Sim {
    pub(super) fn power_write(&mut self, v: u32) {
        let mut v = v as u8;
        if v & 1 != 0 && self.regs[0x29] & 1 == 0 && self.power_ignores > 0 {
            self.power_ignores -= 1;
            v &= !1;
        }
        if v & 1 != 0 && self.regs[0x29] & 1 == 0 {
            let sel = (v >> 1) & 7;
            if !matches!(sel, 5..=7) {
                self.violations.push(format!("bus power on with voltage select {sel}"));
            }
            if sel == 5 && self.cfg.caps & 1 << 26 == 0
                || sel == 6 && self.cfg.caps & 1 << 25 == 0
                || sel == 7 && self.cfg.caps & 1 << 24 == 0
            {
                self.violations.push(format!("voltage select {sel} the host does not offer"));
            }
            self.power_ons.push(sel);
            self.dev().power(true);
        }
        if v & 1 == 0 {
            self.dev().power(false);
        }
        self.regs[0x29] = v;
    }
}
