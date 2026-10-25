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

//! The sink's side of what the driver writes: GMBUS0 selects the pins and
//! opens a try (0 closes it), GMBUS3 carries a write's bytes, and a stop
//! puts the segment pointer back at 0 (GMBUS1's commands in
//! `sink_command.rs`).

use super::bus::Cmd;
use super::sink::{Sink, Try};

impl Sink<'_> {
    pub(super) fn select(&mut self, v: u32) {
        let now = self.clock.now.get();
        if v != 0 && self.bus.sel == 0 {
            let fault = self.faults.get(self.log.len()).copied().unwrap_or_default();
            self.log.push(Try { fault, t0: now, ..Default::default() });
        }
        if let (0, Some(t)) = (v, self.log.last_mut()) {
            t.t1 = now;
        }
        self.bus.sel = v;
    }

    pub(super) fn data(&mut self, v: u32) {
        self.bus.data = v;
        if let (Some(c), Some(t)) = (self.bus.cmd, self.log.last_mut()) {
            let room = c.len.saturating_sub(t.sent.len()).min(4);
            if !c.read {
                t.sent.extend_from_slice(&v.to_le_bytes()[..room]);
            }
        }
    }

    pub(super) fn stop(&mut self) {
        if let (Some(Cmd { addr: 0x37, read: false, .. }), Some(t)) =
            (self.bus.cmd, self.log.last())
        {
            self.bus.ci_last = t.sent.clone();
        }
        (self.bus.segment, self.bus.pointed, self.bus.cmd) = (0, false, None);
    }
}
