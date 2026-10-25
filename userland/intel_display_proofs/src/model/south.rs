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

//! The power wells and the panel power sequencer in the model: a request
//! is answered a few reads later, as the hardware takes its time; nothing
//! answers when `dead` is set; the sequencer ignores a write without its
//! key.

use super::gen9::Gen9;
use crate::engine::pcode::{DATA, MAILBOX, READY};
use crate::engine::power::PWR_WELL_CTL2;
use crate::engine::pps::{KEY, ON, PP_CONTROL, PP_STATUS, TARGET_ON};

impl Gen9 {
    pub(super) fn tick_pending(&mut self) {
        for p in self.pending.iter_mut() {
            p.0 = p.0.saturating_sub(1);
        }
        let due: Vec<_> = self.pending.iter().filter(|p| p.0 == 0).copied().collect();
        self.pending.retain(|p| p.0 != 0);
        for (_, off, v) in due {
            self.regs.insert(off, v);
        }
    }

    pub(super) fn south_write(&mut self, off: u32, v: u32) {
        if self.dead {
            return;
        }
        if off == PWR_WELL_CTL2 {
            // Each request bit's state bit, one below, follows it.
            let states = (v & 0xAAAA_AAAA) >> 1;
            let req = v & 0xAAAA_AAAA;
            self.pending.push((3, off, req | states));
        }
        if off == MAILBOX && v & READY != 0 {
            // Latencies 2, 4, 8, 16 then 24, 32, 48 and none.
            let answer = if self.get(DATA) == 0 { 0x1008_0402 } else { 0x0030_2018 };
            self.pending.push((2, DATA, answer));
            self.pending.push((3, MAILBOX, v & !READY));
        }
        if off == PP_CONTROL {
            if v & 0xFFFF_0000 != KEY {
                let old = *self.live.get(&off).unwrap_or(&0);
                self.regs.insert(off, old);
                return;
            }
            self.live.insert(off, v);
            let busy = if v & TARGET_ON != 0 { 1 << 28 } else { 2 << 28 };
            self.regs.insert(PP_STATUS, busy);
            let done = if v & TARGET_ON != 0 { ON } else { 0 };
            self.pending.push((4, PP_STATUS, done));
        }
    }
}
