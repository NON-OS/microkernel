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

//! Ending what has exited, and telling each parent: the ended child waits
//! until it is waited for, and a wait4 parked for it is answered.

use nonos_libc::mk_foreign_reply;

use super::family::Family;
use super::pid_ns::PidNs;
use super::pid_out::value_out;
use crate::linux::abi::nr;
use crate::linux::call::reap_one;
use crate::linux::guest::Guest;

impl Family {
    /// End every process that asked to, deliver the signals that follow,
    /// and go again while that ends anything more.
    pub fn reap(&mut self) {
        self.settle_pipes();
        loop {
            let ended = self.end_exited();
            self.settle_signals();
            if !ended && !self.guests.iter().any(|g| g.exited.is_some()) {
                return;
            }
        }
    }
}

/// Tell `p` that `gone` ended with `code`: kept until it is waited for, and a
/// parked wait4 answered with its pid in the guest's numbering.
pub fn tell_parent(p: &mut Guest, gone: &Guest, code: i32, ns: &mut PidNs) {
    p.ended.push((gone.pid, code));
    if let Some((want, status, tid)) = p.waiting {
        if let Some(value) = reap_one(p, want, status) {
            p.waiting = None;
            let _ = mk_foreign_reply(tid, value_out(ns, nr::WAIT4, value));
        }
    }
}
