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

/*
 * What a process leaves behind when it exits, settled on its last call,
 * while the family still holds it.
 */

use nonos_libc::ForeignFrame;

use super::family::Family;
use crate::linux::abi::nr;
use crate::linux::file;

impl Family {
    /*
     * A process about to exit: what it used goes to its parent's
     * RUSAGE_CHILDREN once waited for, its POSIX locks go, and every file
     * the family is writing reaches the store, as the exit's closes would.
     */
    pub(super) fn note_exit(&self, i: usize, frame: &ForeignFrame) {
        let g = &self.guests[i];
        let leaving = frame.nr == nr::EXIT_GROUP || (frame.nr == nr::EXIT && g.threads.is_empty());
        if !leaving {
            return;
        }
        let parent = self.guests.iter().find(|p| p.children.contains(&g.pid)).map_or(0, |p| p.pid);
        let mut used = crate::linux::call::usage_of(g);
        let kids = file::cpu::children(g.pid, |c| !g.children.contains(&c));
        used.user += kids.user;
        used.system += kids.system;
        file::cpu::ended(g.pid, parent, used);
        file::locks_exiting(g.pid);
        let _ = file::flush_all();
    }
}
