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

//! A running thread the kernel stopped at a tick because a signal was raised
//! for it: it takes what it may on the registers it stopped with, or runs on
//! exactly where it was.

use nonos_libc::{mk_foreign_context, mk_foreign_reply, ForeignRegs};

use super::family::Family;

impl Family {
    pub(super) fn interrupted(&mut self, tid: u32) {
        let mut regs: ForeignRegs = [0; 18];
        if mk_foreign_context(tid, &mut regs) != 0 {
            let _ = mk_foreign_reply(tid, 0);
            return;
        }
        let Some(g) = self.guests.iter_mut().find(|g| g.owns(tid)) else {
            let _ = mk_foreign_reply(tid, 0);
            return;
        };
        /* No call is being answered: a handler returns to the thread's own rax. */
        if !super::deliver::deliver_on(g, tid, regs) {
            let _ = mk_foreign_reply(tid, 0);
        }
    }
}
