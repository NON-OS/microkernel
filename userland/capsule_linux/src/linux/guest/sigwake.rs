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

//! Waking the thread a signal was raised for. One parked in a call takes it
//! when that call is answered or its wait ends; one running its own code makes
//! no call at all, so the kernel is asked to stop it at its next tick and hand
//! it back (`serve::family_interrupt`), where it takes the signal on the
//! registers it stopped with.

use super::handle::Guest;
use super::siginfo::SigInfo;

impl Guest {
    /// Queue `info` for `tid`, or for the process when `tid` is 0, and wake the
    /// thread that will take it. False when nothing was queued.
    pub fn raise_and_wake(&mut self, tid: u32, info: SigInfo) -> bool {
        if !self.signals.raise(tid, info) {
            return false;
        }
        let target = if tid == 0 { self.pid } else { tid };
        let _ = nonos_libc::mk_foreign_interrupt(target);
        true
    }

    /*
     * After an answer that could not carry a handler, a woken futex or a
     * return from a handler, a thread with a signal still waiting for it or
     * for its process is marked for a tick stop and takes it there. A stop
     * that finds nothing it may take only resumes the thread.
     */
    pub fn rearm(&self, tid: u32) {
        if self.signals.pending.iter().any(|(t, _)| *t == tid || *t == 0) {
            let _ = nonos_libc::mk_foreign_interrupt(tid);
        }
    }
}
