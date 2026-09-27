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

//! What a forked child starts with.
//!
//! A fork is a second process, so the child gets its own copy of what this
//! personality tracks for one: the descriptor table, the break, the mapping
//! plan and what it covers, the cwd, the thread pointer. Descriptors are
//! dup'd, sharing what they name as Linux shares an open file. Pipe buffers
//! are not here: they belong to the family, so both ends of a fork see one.
//! The display is not inherited; a child that wants a window connects.

use alloc::vec::Vec;

use super::fd::Fd;
use super::handle::Guest;

impl Guest {
    pub fn fork_state(&self, child: u32) -> Guest {
        let mut g = Guest::new(child);
        g.brk = self.brk;
        g.mmap_next = self.mmap_next;
        // dup clears close-on-exec; fork keeps it, and exec is where it counts.
        g.fds = self.fds.iter().map(|f| Fd { cloexec: f.cloexec, ..Fd::clone_of(f) }).collect();
        g.regions = self.regions.clone();
        g.pipes = Vec::new();
        g.handlers = self.handlers;
        g.cwd = self.cwd.clone();
        g.automap = self.automap.clone();
        g.fs_base = self.fs_base;
        g.parent = self.parent;
        g.pgid = self.pgid;
        g.sid = self.sid;
        g.umask = self.umask;
        g.links = self.links.clone();
        g
    }
}
