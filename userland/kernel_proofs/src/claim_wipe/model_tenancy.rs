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

//! A tenancy begun and a process's exit begun: at admit, the runs its driver
//! will map and whether a sibling thread will also give the function back;
//! once its main thread is done in user code, its exit, at once unless a
//! kill already began it.

use super::phase_rule::MS;
use super::world::World;
use super::world_thread::{Path, Thread};

impl World {
    pub(super) fn start_tenancy(&mut self, p: usize, t: &mut Thread) {
        self.procs[p].maps = 1 + self.draw(4) as u8;
        (t.path, t.step) = (Path::Hold, 0);
        if self.chance(4) {
            let at = self.now + MS * self.draw(301);
            self.threads[3 * p + 1] = Thread { path: Path::Give, at, ..Thread::default() };
        }
    }

    /// The exit of a process done in user code: now, if not already begun.
    pub(super) fn exit_now(&mut self, p: usize) {
        let (now, e) = (self.now, &mut self.threads[3 * p + 2]);
        match e.path {
            Path::Off => *e = Thread { path: Path::Exit, at: now, ..Thread::default() },
            Path::Exit if e.step == 0 => e.at = e.at.min(now),
            _ => {}
        }
    }
}
