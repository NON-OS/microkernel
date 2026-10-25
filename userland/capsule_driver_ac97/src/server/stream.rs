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

//! The one output stream: whether it runs, the queue it plays from, which
//! period the refill writes next and how the periods went.

use crate::audio::{PcmQueue, Periods};
use crate::setup::Driver;
use crate::{codec, engine, say};

#[derive(Default)]
pub struct Stream {
    pub running: bool,
    pub q: PcmQueue,
    pub wpos: usize,
    pub periods: Periods,
}

impl Stream {
    /// Ring silenced, engine started on it, outputs opened. False when the
    /// engine did not start; it is then left stopped and the outputs muted.
    pub fn start(&mut self, d: &mut Driver) -> bool {
        if self.running {
            return true;
        }
        d.silence();
        self.wpos = 0;
        self.periods = Periods::new();
        if !engine::start(&d.bus, d.bdl_dev()) {
            let _ = engine::stop(&d.bus);
            return false;
        }
        codec::open(&d.mixer, true);
        self.running = true;
        true
    }

    /// Outputs muted, engine stopped, queue emptied, the run's totals said.
    pub fn stop(&mut self, d: &mut Driver) {
        if !self.running {
            return;
        }
        codec::open(&d.mixer, false);
        let _ = engine::stop(&d.bus);
        self.q.clear();
        self.running = false;
        say::totals(&self.periods);
    }
}
