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

//! A watch over the model for deaths and failures (G-72), written from the
//! test's own reading: the frames the driver gave back, and after every
//! register write a check that no plane or cursor that is enabled, armed
//! or latched, reaches one of them through the GGTT. A test may also turn
//! the engine dead at step k (a failure the driver must undo), or kill the
//! driver there (a panic caught by the test, as a capsule killed then).

use super::gen9::Gen9;

#[derive(Default)]
pub struct Watch {
    /// IOVA ranges given back.
    pub freed: Vec<(u64, u64)>,
    /// Times a plane or cursor reached a freed frame.
    pub reached: u32,
    pub steps: u32,
    /// The steps that were register writes (the rest: GGTT entries).
    pub regs_at: Vec<u32>,
    /// From this step the engine is dead and latches nothing, for
    /// `dead_for` steps (0: for good).
    pub dead_at: Option<u32>,
    pub dead_for: u32,
    pub period_was: u32,
    pub die_at: Option<u32>,
}

impl Gen9 {
    /// After every write: count the step, turn dead or die on cue, check.
    pub fn step(&mut self, reg: bool) {
        self.watch.steps += 1;
        let n = self.watch.steps;
        if reg {
            self.watch.regs_at.push(n);
        }
        if self.watch.dead_at == Some(n) {
            (self.dead, self.watch.period_was, self.period) = (true, self.period, 0);
        }
        let back = self.watch.dead_at.map(|k| k + self.watch.dead_for);
        if self.watch.dead_for > 0 && back == Some(n) {
            (self.dead, self.period) = (false, self.watch.period_was);
        }
        self.check();
        if self.watch.die_at == Some(n) {
            panic!("the driver is killed at step {n}");
        }
    }

    pub fn check(&mut self) {
        if self.reaching().iter().any(|a| self.watch.freed.iter().any(|f| (f.0..f.1).contains(a))) {
            self.watch.reached += 1;
        }
    }
}
