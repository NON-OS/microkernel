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

//! GMBUS1 as the sink takes it: a clear or a stop ends what was going on;
//! otherwise a read or a write starts at the address, acknowledged only by
//! a device that is there on the selected pins.

use super::bus::Cmd;
use super::sink::{Eddc, Sink};
use crate::engine::gmbus_regs::{CYCLE_INDEX, CYCLE_STOP, SW_CLR_INT, SW_RDY};

impl Sink<'_> {
    pub(super) fn command(&mut self, v: u32) {
        if v & SW_CLR_INT != 0 || v & SW_RDY == 0 {
            self.bus.cmd = None;
            return;
        }
        if v & CYCLE_STOP != 0 {
            return self.stop();
        }
        let (addr, read, len) = ((v >> 1) & 0x7F, v & 1 == 1, ((v >> 16) & 0x1FF) as usize);
        let index = if v & CYCLE_INDEX != 0 { (v >> 8) & 0xFF } else { 0 };
        let ack = self.bus.sel & 7 == self.pins
            && match (addr, read) {
                (0x30, false) => self.eddc != Eddc::Nak,
                (0x50, true) => true,
                (0x37, false) => !self.ci_nak,
                (0x37, true) => self.ci.is_some(),
                _ => false,
            };
        self.bus.cmd = Some(Cmd { addr, read, len });
        self.bus.words = 0;
        let first = self.bus.data.to_le_bytes();
        let Some(t) = self.log.last_mut() else { return };
        t.nak |= !ack;
        let fault = t.fault;
        if read {
            // The wait phase logged is the read's, not the pointer's before it.
            (t.read, t.phase) = (Some((addr, index, len)), false);
            self.bus.stream = self.stream(addr, index, len, fault);
        } else {
            t.sent = first[..len.min(4)].to_vec();
            if addr == 0x30 {
                t.segment = Some((first[0], ack));
                (self.bus.segment, self.bus.pointed) = (first[0], ack);
            }
        }
    }
}
