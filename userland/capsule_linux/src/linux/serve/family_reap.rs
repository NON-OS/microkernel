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

//! Ending what has exited, and telling each parent.

use nonos_libc::{mk_foreign_reply, mk_kill};

use super::family::Family;
use crate::linux::call::reap_one;

const SIGKILL: u64 = 9;

impl Family {
    /// End every process that asked to, and tell its parent.
    pub fn reap(&mut self) {
        self.settle_pipes();
        while let Some(i) = self.guests.iter().position(|g| g.exited.is_some()) {
            let gone = self.guests.remove(i);
            let code = gone.exited.unwrap_or(0);
            for tid in gone.threads.iter().chain([gone.pid].iter()) {
                let _ = mk_kill(*tid as u64, SIGKILL);
            }
            if gone.pid == self.root {
                self.root_code = code;
            }
            let Some(p) = self.guests.iter_mut().find(|g| g.children.contains(&gone.pid)) else {
                continue;
            };
            p.ended.push((gone.pid, code));
            if let Some((want, status, tid)) = p.waiting {
                if let Some(value) = reap_one(p, want, status) {
                    p.waiting = None;
                    let _ = mk_foreign_reply(tid, value);
                }
            }
        }
    }
}
