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

//! The signal mask a ppoll, pselect6, epoll_pwait or epoll_pwait2 waits
//! under. It replaces the thread's own for the length of the wait, as
//! sigsuspend's does: a call that answers puts the old one back first, so a
//! signal only the wait let through stays pending; one a signal ends keeps
//! it through the handler, whose return puts the old one back, as Linux's
//! restore_saved_sigmask_unless does.

use crate::linux::abi::{errno, nr, nr_path as np};
use crate::linux::call::wait_mask;
use crate::linux::guest::Guest;

/// The call's mask, read from where each call keeps it, or the errno a bad
/// one earns.
pub fn read(guest: &Guest, number: u64, a: &[u64; 6]) -> Result<Option<u64>, u64> {
    let (at, size) = match number {
        np::PPOLL => (a[3], a[4]),
        nr::EPOLL_PWAIT | nr::EPOLL_PWAIT2 => (a[4], a[5]),
        /* pselect6's last argument points at { const sigset_t *ss; size_t ss_len; }. */
        np::PSELECT6 if a[5] != 0 => {
            let raw = guest.read(a[5], 16).ok_or(errno::fail(errno::EFAULT))?;
            let word = |at: usize| raw[at..at + 8].try_into().map_or(0, u64::from_le_bytes);
            (word(0), word(8))
        }
        _ => return Ok(None),
    };
    wait_mask(guest, at, size).map_err(errno::fail)
}

/// Wait under `mask`, keeping the thread's own to put back.
pub fn enter(guest: &mut Guest, tid: u32, mask: u64) {
    let old = guest.signals.blocked(tid);
    guest.signals.set_blocked(tid, mask);
    guest.signals.thread(tid).saved = Some(old);
}

/// The thread's own mask back, for a wait of `number` that answers.
pub fn leave(guest: &mut Guest, tid: u32, number: u64) {
    if !matches!(number, np::PPOLL | np::PSELECT6 | nr::EPOLL_PWAIT | nr::EPOLL_PWAIT2) {
        return;
    }
    if let Some(old) = guest.signals.thread(tid).saved.take() {
        guest.signals.set_blocked(tid, old);
    }
}
