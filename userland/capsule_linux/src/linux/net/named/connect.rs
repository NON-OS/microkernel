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

//! `connect` on a Unix socket. A connect to the display's path
//! turns the descriptor into the display connection this capsule serves
//! itself (`unix`); every other name is the family's own.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;
use crate::linux::net::sock::{self, Link, Proto};
use crate::linux::net::sockaddr::{self, AF_UNSPEC};
use crate::linux::unix;

use super::addr::{unix_addr, UAddr};
use super::display::display;
use super::name::{find, resolve};

pub fn connect(guest: &mut Guest, fd: u64, id: u32, proto: Proto, at: u64, len: u64) -> u64 {
    if proto == Proto::Dgram && matches!(sockaddr::read(guest, at, len), Ok((AF_UNSPEC, _))) {
        sock::with(|t| t.get_mut(id).map(|s| (s.peer, s.upeer, s.connected) = (None, None, false)));
        return errno::ok(0);
    }
    let ua = match unix_addr(guest, at, len) {
        Ok(ua) => ua,
        Err(e) => return e,
    };
    if let (Proto::Stream, UAddr::Path(p)) = (proto, &ua) {
        if unix::is_display(p) {
            return display(guest, fd, at, len);
        }
    }
    let Some(name) = resolve(guest, &ua) else {
        return errno::fail(errno::EINVAL);
    };
    let target = match find(&name, proto) {
        Ok(t) => t,
        Err(e) => return errno::fail(e),
    };
    sock::with(|t| {
        let s = t.get_mut(id).ok_or(errno::EBADF)?;
        match proto {
            Proto::Stream if s.connected => Err(errno::EISCONN),
            Proto::Stream if s.listening => Err(errno::EINVAL),
            /* A Unix connect completes in the caller's call, blocking or not. */
            Proto::Stream => match t.join(id, target) {
                Link::Done => Ok(()),
                Link::Full => Err(errno::EAGAIN),
                Link::Refused => Err(errno::ECONNREFUSED),
            },
            Proto::Dgram => {
                (s.peer, s.upeer, s.connected) = (Some(target), Some(name), true);
                Ok(())
            }
        }
    })
    .map_or_else(errno::fail, |()| errno::ok(0))
}
