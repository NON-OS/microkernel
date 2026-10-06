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

//! Names for pids, from one read of the process table: the same call the
//! checks above make, so a name here is the name the kernel holds.

use alloc::vec::Vec;

use nonos_libc::PROC_NAME_LEN;

use super::procs::{each, name_of};

pub(super) struct Names {
    rows: Vec<(u32, [u8; PROC_NAME_LEN], usize)>,
}

impl Names {
    /// Empty when the kernel would not answer: every pid then reads unnamed.
    pub(super) fn read() -> Names {
        let mut rows = Vec::new();
        each(|e| {
            let name = name_of(e);
            let mut held = [0u8; PROC_NAME_LEN];
            held[..name.len()].copy_from_slice(name);
            rows.push((e.pid, held, name.len()));
        });
        Names { rows }
    }

    /// The name and its length, or an empty name for a pid the table lacks.
    pub(super) fn of(&self, pid: u32) -> ([u8; PROC_NAME_LEN], usize) {
        match self.rows.iter().find(|(p, _, _)| *p == pid) {
            Some((_, name, len)) => (*name, *len),
            None => ([0; PROC_NAME_LEN], 0),
        }
    }
}
