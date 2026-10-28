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

//! A pair becoming a session's controlling terminal, and ceasing to be.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;
use crate::linux::serve::guest_pid;

use super::cell::with;
use super::note::{SIGCONT, SIGHUP};
use super::pair::Pair;
use super::table::To;

/// `f` done to pair `n`: zero, or EIO with the pair gone.
pub(super) fn on(n: u32, f: impl FnOnce(&mut Pair)) -> u64 {
    with(|t| match t.get(n) {
        Some(p) => {
            f(p);
            errno::ok(0)
        }
        None => errno::fail(errno::EIO),
    })
}

/// TIOCSCTTY, as foot's child asks after setsid: only a session leader with
/// no other terminal, onto a pair no other session has. Its group becomes
/// the foreground one.
pub(super) fn ctty(guest: &Guest, n: u32, session: u32) -> u64 {
    if guest.pid != guest.sid {
        return errno::fail(errno::EPERM);
    }
    match with(|t| t.controlling(guest.sid)) {
        Some(m) if m == n => return errno::ok(0),
        Some(_) => return errno::fail(errno::EPERM),
        None if session != 0 => return errno::fail(errno::EPERM),
        None => {}
    }
    let fg = guest_pid(guest.pgid);
    on(n, |p| (p.session, p.fg) = (guest.sid, fg))
}

/// TIOCNOTTY from the session: its leader giving the terminal up sends the
/// foreground group SIGHUP and SIGCONT, as Linux's disassociate_ctty does.
pub(super) fn notty(guest: &Guest, n: u32) -> u64 {
    if guest.pid != guest.sid {
        return errno::ok(0);
    }
    with(|t| {
        let fg = t.get(n).map(|p| core::mem::replace(&mut p.fg, 0));
        if let Some(p) = t.get(n) {
            p.session = 0;
        }
        let fg = fg.filter(|&g| g != 0);
        t.signals
            .extend(fg.into_iter().flat_map(|g| [(To::Group(g), SIGHUP), (To::Group(g), SIGCONT)]));
    });
    errno::ok(0)
}
