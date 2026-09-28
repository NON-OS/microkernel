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
//! for it: deliver it now, or let the thread run on exactly where it was.

use nonos_libc::{mk_foreign_context, mk_foreign_reply, ForeignRegs};

use super::family::Family;

/// rax in the register word order.
const RAX: usize = 13;

impl Family {
    pub(super) fn interrupted(&mut self, tid: u32) {
        let mut regs: ForeignRegs = [0; 18];
        let rax = if mk_foreign_context(tid, &mut regs) == 0 { regs[RAX] } else { 0 };
        let Some(g) = self.guests.iter_mut().find(|g| g.owns(tid)) else {
            let _ = mk_foreign_reply(tid, 0);
            return;
        };
        // No call is being answered: the handler returns to the thread's own rax.
        if !super::deliver::maybe_deliver(g, tid, rax) {
            let _ = mk_foreign_reply(tid, 0);
        }
    }
}
