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

//! `kill`, `tkill` and `tgkill`, for the guest's own threads and children.
//! A signal the process catches is queued and delivered on that thread's next
//! return; one whose default is to ignore is dropped; a fatal default ends it.

use nonos_libc::mk_kill;

use crate::linux::abi::errno;
use crate::linux::guest::siginfo::{SigInfo, SI_USER};
use crate::linux::guest::sigstate::NSIG;
use crate::linux::guest::Guest;

/// Signals whose default action is to be ignored: child status, urgent data,
/// window size, and a continue with nothing stopped.
const IGNORED_DEFAULT: [u64; 4] = [17, 23, 28, 18];

pub fn kill(guest: &mut Guest, pid: u64, signo: u64) -> u64 {
    let target = pid as u32;
    // A guest may signal itself, its threads and its children, nothing else.
    if !guest.owns(target) && !guest.children.contains(&target) {
        return errno::fail(errno::ESRCH);
    }
    if signo == 0 {
        return errno::ok(0); // an existence check, not a signal
    }
    if signo > NSIG as u64 {
        return errno::fail(errno::EINVAL);
    }
    let act = guest.signals.action(signo as usize).unwrap_or_default();
    if act.catches() {
        let _ = guest.signals.raise(target, SigInfo::from(signo as u8, SI_USER, guest.pid));
        return errno::ok(0);
    }
    if act.ignores() || IGNORED_DEFAULT.contains(&signo) {
        return errno::ok(0);
    }
    terminate(guest, target, signo)
}

/// The default action of an uncaught, non-ignored signal is to end the thread.
fn terminate(guest: &mut Guest, target: u32, signo: u64) -> u64 {
    guest.forget_thread(target);
    guest.threads.retain(|t| *t != target);
    match mk_kill(target as u64, signo) {
        n if n < 0 => errno::fail(errno::EPERM),
        _ => errno::ok(0),
    }
}
