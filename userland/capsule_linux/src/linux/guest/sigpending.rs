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

//! A caught signal still waiting after a thread was answered without it: the
//! answer to a woken futex or to rt_sigreturn cannot carry a handler, so the
//! thread is marked instead, the kernel stops it at its next tick in its own
//! code, and it is delivered there.

use super::handle::Guest;
use super::sigqueue::Signals;

impl Signals {
    /// Whether `take_caught` would find a signal for `tid`.
    pub fn has_caught(&self, tid: u32) -> bool {
        self.pending.iter().any(|(t, s)| *t == tid && self.actions[*s as usize - 1].catches())
    }
}

impl Guest {
    /// After an answer that entered no handler, mark `tid` for a tick stop if
    /// a caught signal is still waiting for it.
    pub fn rearm(&self, tid: u32) {
        if self.signals.has_caught(tid) {
            let _ = nonos_libc::mk_foreign_interrupt(tid);
        }
    }
}
