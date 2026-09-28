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

//! Ending each process that has exited: its threads are killed and its parent
//! is told as family_reap tells it.

use nonos_libc::mk_kill;

use super::family::Family;
use super::family_reap::tell_parent;

const SIGKILL: u64 = 9;

impl Family {
    pub(super) fn end_exited(&mut self) -> bool {
        let mut any = false;
        while let Some(i) = self.guests.iter().position(|g| g.exited.is_some()) {
            any = true;
            let gone = self.guests.remove(i);
            let code = gone.exited.unwrap_or(0);
            for tid in gone.threads.iter().chain([gone.pid].iter()) {
                let rc = mk_kill(*tid as u64, SIGKILL);
                if rc < 0 {
                    /* Refused, it runs on after its process ended. */
                    let line = alloc::format!(
                        "[LINUX] kill refused: pid {tid} outlives its process, errno {}\n",
                        -rc
                    );
                    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
                }
            }
            if gone.pid == self.root {
                self.root_code = code;
            }
            let Some(p) = self.guests.iter_mut().find(|g| g.children.contains(&gone.pid)) else {
                continue;
            };
            tell_parent(p, &gone, code, &mut self.ns);
        }
        any
    }
}
