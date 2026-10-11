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

//! The rules a command must keep as it is issued: no inhibited line, and
//! the response flags the card in the slot needs for it.

use super::sim::Sim;

impl Sim {
    pub(super) fn check_issue(&mut self, index: u8, word: u16) {
        let ps = self.present();
        let abort = word >> 6 & 3 == 3;
        if ps & 1 != 0 || (ps & 2 != 0 && (word & 0x20 != 0 || word & 3 == 3) && !abort) {
            self.violate(format!("CMD{index} issued while inhibited"));
        }
        match self.dev_ref().flags(index) {
            Some(f) if f == word & 0xff => {}
            Some(f) => self.violate(format!("CMD{index} flags {:#x}, want {:#x}", word & 0xff, f)),
            None => self.violate(format!("CMD{index} is not a command the driver should send")),
        }
    }
}
