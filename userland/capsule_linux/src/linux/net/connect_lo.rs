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

//! A stream connect to 127.0.0.0/8, which the family completes in the
//! caller's own call, as Linux's loopback does.

use crate::linux::abi::errno;

use super::sock::{self, Addr, Link};

pub fn loopback(id: u32, to: Addr, nonblock: bool) -> u64 {
    sock::with(|t| {
        let Some(s) = t.get_mut(id) else {
            return errno::fail(errno::EBADF);
        };
        if s.connecting {
            return errno::fail(errno::EALREADY);
        }
        if s.connected || s.listening || s.svc.is_some() {
            return errno::fail(errno::EISCONN);
        }
        s.error = 0;
        let bound_here = s.local.is_none();
        if let Err(e) = t.autobind(id) {
            return errno::fail(e);
        }
        let linked = t.link(id, to);
        // A full listener keeps a non-blocking connect until accept makes
        // room, as Linux's SYN_SENT does.
        if let (Link::Full, true, Some(l)) = (&linked, nonblock, t.listener(to)) {
            t.wait_room(id, l);
            return errno::fail(errno::EINPROGRESS);
        }
        // A connect that fails gives back the port it bound.
        if !matches!(linked, Link::Done) && bound_here {
            if let Some(s) = t.get_mut(id) {
                s.local = None;
            }
        }
        match linked {
            Link::Done if nonblock => errno::fail(errno::EINPROGRESS),
            Link::Done => errno::ok(0),
            Link::Refused if nonblock => {
                if let Some(s) = t.get_mut(id) {
                    s.error = errno::ECONNREFUSED;
                }
                errno::fail(errno::EINPROGRESS)
            }
            Link::Refused => errno::fail(errno::ECONNREFUSED),
            Link::Full => errno::fail(errno::EAGAIN),
        }
    })
}
