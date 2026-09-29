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

//! Answering wait4 and waitid. A child that has ended and fits is reported,
//! and reaped unless WNOWAIT; with none ended, WNOHANG answers 0, and a caller
//! with no child that could ever fit gets ECHILD. The family answers, since a
//! wait by group needs every child's group, which only it can see.

use nonos_libc::mk_foreign_reply;

use super::family::Family;
use crate::linux::guest::Guest;

/// Reply to a parked thread, or enter the handler of a signal it now takes.
pub fn answer(g: &mut Guest, tid: u32, value: u64) {
    if !super::deliver::maybe_deliver(g, tid, value) {
        let _ = mk_foreign_reply(tid, value);
    }
}

impl Family {
    pub(super) fn settle_child_waits(&mut self) {
        for i in 0..self.guests.len() {
            for w in core::mem::take(&mut self.guests[i].signals.childwaits) {
                match self.try_wait(i, &w) {
                    Some(v) => answer(&mut self.guests[i], w.tid, v),
                    None => self.guests[i].signals.childwaits.push(w),
                }
            }
        }
    }
}
