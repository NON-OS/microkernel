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

//! Present State, and the transfer complete a busy phase raises later.

use super::sim::Sim;

impl Sim {
    pub(super) fn present(&mut self) -> u32 {
        let mut ps = 1 << 17 | 0xf << 20 | 1 << 24;
        if self.cfg.cd_inserted || self.regs[0x28] & 0xc0 == 0xc0 {
            ps |= 1 << 16 | 1 << 18;
        }
        if self.tc_at.is_some() {
            ps |= 1 << 1 | 1 << 2;
        }
        // Write Protect Switch Pin Level: set when the card may be written.
        if !self.cfg.write_protected {
            ps |= 1 << 19;
        }
        ps
    }

    pub(super) fn poll_tc(&mut self) {
        if let Some(t) = self.tc_at {
            if self.now() >= t {
                self.tc_at = None;
                self.int |= 1 << 1;
            }
        }
    }
}
