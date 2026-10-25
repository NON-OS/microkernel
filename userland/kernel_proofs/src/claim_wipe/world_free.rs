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

//! held_run.rs `free_now`: a run scrubbed and given back. A run freed while
//! its function can still reach it, or freed twice, breaks I2. The function
//! reaches a run its queue holds while it may master the bus, or always if
//! it reaches memory past Bus Master; a confined run, unmapped from its
//! domain at teardown, it never reaches.

use super::bits_rule::BME;
use super::world::World;

impl World {
    pub(super) fn free_now(&mut self, id: usize) {
        self.runs[id].frees += 1;
        if self.runs[id].frees > 1 {
            self.bad(2, "a run freed twice");
        }
        if self.reaches(id) {
            self.bad(2, "a run freed while its function can reach it");
        }
    }

    pub(super) fn reaches(&self, id: usize) -> bool {
        let d = &self.dev;
        let on = d.cmd & BME != 0 || d.past_bm;
        !self.runs[id].confined && on && d.queue.contains(&id)
    }
}
