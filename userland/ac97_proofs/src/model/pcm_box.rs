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

//! The PCM out box's control register: a register reset, a stop that
//! halts the engine, and a start that fetches the first descriptor.

use super::ich::Ich;

const DCH: u16 = 1;
const RPBM: u8 = 1;
const RR: u8 = 2;

impl Ich {
    pub(super) fn control(&mut self, v: u8) {
        if v & RR != 0 {
            if self.opt.rr_sticks {
                self.cr |= RR;
                return;
            }
            (self.bdbar, self.civ, self.lvi, self.piv, self.picb) = (0, 0, 0, 0, 0);
            self.sr = DCH;
            self.cr &= 0x1C;
        } else if v & RPBM == 0 {
            self.cr = v & 0x1F;
            self.sr |= DCH;
        } else {
            self.cr = v & 0x1F;
            self.advance();
            self.sr &= !DCH;
        }
    }
}
