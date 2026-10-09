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
 * A client that ends without closing its handles leaves them here, and
 * nothing else closed them: a client that crashed in a loop filled the
 * table, and every later open on the machine was refused until a reboot.
 * The handles of an owner that no longer runs are closed as its own close
 * would have closed them. Owner 0 is the kernel speaking for itself, which
 * does not end, and which mk_pid_alive never names as alive.
 */

use super::types::Store;

/// The owner the kernel's own requests carry.
const KERNEL: u32 = 0;

impl Store {
    /// How many handles `owner` holds open.
    pub fn held_by(&self, owner: u32) -> usize {
        self.fds.iter().flatten().filter(|fd| fd.owner_pid == owner).count()
    }

    /// Close every handle whose owner `alive` says has ended, and say how
    /// many were closed.
    pub fn close_ended(&mut self, alive: impl Fn(u32) -> bool) -> usize {
        let mut closed = 0;
        for slot in self.fds.iter_mut() {
            if slot.as_ref().is_some_and(|fd| fd.owner_pid != KERNEL && !alive(fd.owner_pid)) {
                *slot = None;
                closed += 1;
            }
        }
        closed
    }
}

/*
 * The Linux personality keeps each run's files under /linux-private/<id>,
 * which only that run sees, and removes them when the run ends. It aborts on
 * a panic, so a run that crashed left them for good, up to its quota each
 * time. The entries under /linux-private/ of an owner that no longer runs
 * are removed and wiped; the shared /linux-private directory stays.
 */
const PRIVATE: &str = "/linux-private/";

impl Store {
    /// Remove every private entry whose owner `alive` says has ended, and
    /// say how many went.
    pub fn drop_private_of_ended(&mut self, alive: impl Fn(u32) -> bool) -> usize {
        let mut dropped = 0;
        let mut idx = self.files.len();
        while idx > 0 {
            idx -= 1;
            let f = &self.files[idx];
            if f.name.starts_with(PRIVATE) && f.owner != KERNEL && !alive(f.owner) {
                self.remove_at(idx);
                dropped += 1;
            }
        }
        dropped
    }
}
