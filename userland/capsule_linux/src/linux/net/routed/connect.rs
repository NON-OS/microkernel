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

//! A connect, as Linux answers one over TCP. The first try opens at once
//! when it can; otherwise a blocking connect waits (EAGAIN parks it, and
//! its retries come back here), and a non-blocking one is EINPROGRESS and
//! is told by POLLOUT, SO_ERROR or a second connect: EALREADY while it is
//! opening, 0 once it opened, its error if it failed, then EISCONN.

use nonos_libc::mk_uptime_ms;

use super::super::sock::{self, Addr, Backend};
use super::answer::answer;
use super::drive::{drive, handle};
use super::entry::{Dest, Entry, Phase};
use super::{retry, table};
use crate::linux::abi::errno::{self, EALREADY, EBADF, EINPROGRESS, EISCONN};

/// A stream to `dest`, socket `id` naming `to` as its peer.
pub fn dial(id: u32, dest: Dest, to: Addr, nonblock: bool) -> u64 {
    let now = mk_uptime_ms();
    let mut e = Entry::new(Phase::Route { again: now }, retry::until(now));
    e.dest = dest;
    let h = table::insert(e);
    let named = sock::with(|t| {
        let s = t.get_mut(id)?;
        (s.svc, s.remote, s.error, s.opened_late) = (Some(Backend::Routed(h)), Some(to), 0, false);
        Some(())
    });
    if named.is_none() {
        drop(table::remove(h));
        return errno::fail(EBADF);
    }
    answer(id, drive(id, true), nonblock, EINPROGRESS)
}

/// A connect on socket `id` that began one already, or saw one finish since
/// it returned; None for any other socket.
pub fn again(id: u32, nonblock: bool) -> Option<u64> {
    let late = sock::with(|t| t.get_mut(id).map(|s| core::mem::take(&mut s.opened_late)))?;
    if late {
        return Some(errno::ok(0));
    }
    let h = handle(id)?;
    if table::with(h, |e| e.endpoint()).unwrap_or(true) {
        return Some(errno::fail(EISCONN));
    }
    Some(answer(id, drive(id, true), nonblock, EALREADY))
}
