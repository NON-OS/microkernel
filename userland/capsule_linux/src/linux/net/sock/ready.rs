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

//! What a family socket can do now, in poll's bits, as Linux's tcp_poll,
//! udp_poll and unix_poll report it.

use super::cell::with;
use super::types::{Proto, Sock};

const POLLIN: u16 = 0x001;
const POLLOUT: u16 = 0x004;
const POLLERR: u16 = 0x008;
const POLLHUP: u16 = 0x010;
const POLLRDHUP: u16 = 0x2000;

/// The bits for `id`, or None when net.sockets or net.anon holds its state.
pub fn bits(id: u32) -> Option<u16> {
    with(|t| {
        let s = t.get(id).filter(|s| s.svc.is_none())?;
        /* A stream is writable while its peer has room, or once it is gone. */
        let room =
            s.peer.and_then(|p| t.get(p)).is_none_or(|p| p.rx.len() < p.opts.rcvbuf as usize);
        Some(of(s, room))
    })
}

fn of(s: &Sock, room: bool) -> u16 {
    let err = if s.error != 0 { POLLERR } else { 0 };
    if s.proto == Proto::Dgram {
        let rd = if s.grams.is_empty() { 0 } else { POLLIN };
        return err | rd | POLLOUT;
    }
    if s.listening {
        return err | if s.pending.is_empty() { 0 } else { POLLIN };
    }
    /* A connect waiting for room: SYN_SENT, neither readable nor writable. */
    if s.connecting {
        return err;
    }
    /* Never connected, refused, or reset: Linux's TCP_CLOSE. */
    if !s.connected || s.broken {
        let rd = if s.connected { POLLIN | POLLRDHUP } else { 0 };
        return err | rd | POLLOUT | POLLHUP;
    }
    let shut_rd = s.eof || s.rd_shut;
    let mut set = err;
    if !s.rx.is_empty() || shut_rd {
        set |= POLLIN;
    }
    if shut_rd {
        set |= POLLRDHUP;
    }
    if shut_rd && s.wr_shut {
        set |= POLLHUP;
    }
    /* After SHUT_WR a write fails at once, so Linux reports it writable. */
    set | if room || s.wr_shut { POLLOUT } else { 0 }
}
