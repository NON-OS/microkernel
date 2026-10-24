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

//! The cable to the DisplayPort sink. Pulled, the DDI's hot-plug line in
//! the south display's live state drops (`engine::hotplug`) and its AUX
//! times out (the channel's own timer ending a request no sink answers),
//! the sink kept aside as the monitor still is; plugged in,
//! both come back. A test can have the cable pulled by the engine's own
//! write to a register, to pull it in the middle of a sequence.

use super::gen9::Gen9;
use super::sink::Sink;
use crate::engine::aux_regs::{aux_ctl, Ddi, DONE, SEND_BUSY, TIMEOUT_ERROR};
use crate::engine::hotplug::SDEISR;

#[derive(Default)]
pub struct Cable {
    /// Pull the cable when the engine next writes this register.
    pub pull_on: Option<u32>,
    pub pulled: Option<(Ddi, Sink)>,
}

/// DDI B's line is bit 21, C's 22, D's 23.
fn line(ddi: Ddi) -> u32 {
    1 << (20 + ddi as u32)
}

impl Gen9 {
    pub fn plug_in(&mut self, ddi: Ddi, s: Sink) {
        self.sink = Some((ddi, s));
        let v = self.get(SDEISR) | line(ddi);
        self.set(SDEISR, v);
    }

    pub fn pull(&mut self) {
        let Some((ddi, s)) = self.sink.take() else { return };
        let mut st = s.borrow_mut();
        (st.link, st.held) = (None, false);
        drop(st);
        self.cable.pulled = Some((ddi, s));
        let v = self.get(SDEISR) & !line(ddi);
        self.set(SDEISR, v);
    }

    pub(super) fn cable_write(&mut self, off: u32, v: u32) {
        if self.cable.pull_on == Some(off) {
            self.cable.pull_on = None;
            self.pull();
        }
        let empty = self.cable.pulled.as_ref().filter(|_| self.sink.is_none());
        if empty.is_some_and(|(ddi, _)| off == aux_ctl(*ddi)) && v & SEND_BUSY != 0 {
            self.regs.insert(off, DONE | TIMEOUT_ERROR);
        }
    }
}
