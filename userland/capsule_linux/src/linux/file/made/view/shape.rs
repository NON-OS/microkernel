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

/* What a family looks like from inside: its processes' files. */

use alloc::vec::Vec;

use super::proc::Proc;

/* A descriptor as /proc/<pid>/fd shows it. */
#[derive(Clone)]
pub struct Open {
    pub fd: u32,
    /* What readlink says it names. */
    pub target: Vec<u8>,
    pub offset: u64,
    /* O_ACCMODE, O_NONBLOCK and O_CLOEXEC, as fdinfo's flags. */
    pub flags: u64,
    /* The open file description, for a file or a directory. */
    pub desc: Option<u32>,
}

#[derive(Default)]
pub struct View {
    /* The asking process's own number. */
    pub me: u32,
    /* The asking thread's number. */
    pub thread: u32,
    pub procs: Vec<Proc>,
}

impl View {
    pub fn find(&self, ns: u32) -> Option<&Proc> {
        self.procs.iter().find(|p| p.ns == ns)
    }
}
