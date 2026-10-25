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

//! GMBUS2 and GMBUS3 as the sink answers them: a NAK, ready for the next
//! word, or the wait phase after the last, with the try's lie of status;
//! and the next word of the read's stream, logged as taken.

use super::sink::{Fault, Sink};
use crate::engine::gmbus_regs::*;

impl Sink<'_> {
    pub(super) fn status(&mut self) -> u32 {
        let (Some(c), Some(t)) = (self.bus.cmd, self.log.last_mut()) else { return 0 };
        let (w, want) = (self.bus.words, c.len.div_ceil(4));
        match (t.nak, c.read, t.fault) {
            (true, ..) => return NAK | ACTIVE,
            (_, false, _) if t.sent.len() < c.len => return HW_RDY | ACTIVE,
            (_, true, Fault::Nak(k)) if w == k => {
                t.nak = true;
                return NAK | ACTIVE;
            }
            (_, true, Fault::Stall(k)) if w == k => return ACTIVE,
            (_, true, f) if w < want => {
                if let (Fault::Slow(us), false) = (f, self.bus.slowed) {
                    self.clock.now.set(self.clock.now.get() + us);
                    self.bus.slowed = true;
                }
                return HW_RDY | ACTIVE;
            }
            (_, true, Fault::NoPhase) => return ACTIVE,
            _ => {}
        }
        (t.phase, t.phase_at) = (true, self.clock.now.get());
        let over = c.read && t.fault == Fault::Over;
        HW_WAIT_PHASE | ACTIVE | if over { HW_RDY } else { 0 }
    }

    pub(super) fn word(&mut self) -> u32 {
        let (Some(c), Some(t)) = (self.bus.cmd, self.log.last_mut()) else { return 0 };
        if !c.read {
            return 0;
        }
        let at = self.bus.words * 4;
        let w: [u8; 4] = core::array::from_fn(|i| *self.bus.stream.get(at + i).unwrap_or(&0xFF));
        t.got.extend_from_slice(&w);
        (self.bus.words, self.bus.slowed) = (self.bus.words + 1, false);
        u32::from_le_bytes(w)
    }
}
