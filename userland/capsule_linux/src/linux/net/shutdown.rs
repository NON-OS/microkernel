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

//! `shutdown`: one direction or both, on the socket rather than the
//! descriptor, which stays open. A dup'd or inherited descriptor sees it too.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::fd::sock_of;
use super::policy::refuse;
use super::sock::{self, Proto};

const SHUT_RD: u64 = 0;
const SHUT_WR: u64 = 1;
const SHUT_RDWR: u64 = 2;

pub fn shutdown(guest: &Guest, fd: u64, how: u64) -> u64 {
    if how > SHUT_RDWR {
        return errno::fail(errno::EINVAL);
    }
    let id = match sock_of(guest, fd) {
        Ok(id) => id,
        Err(e) => return e,
    };
    let (rd, wr) = (how != SHUT_WR, how != SHUT_RD);
    sock::with(|t| {
        let Some(s) = t.get_mut(id) else {
            return errno::fail(errno::EBADF);
        };
        if s.svc.is_some() {
            return refuse(
                "shutdown of a stream outside the family: net.sockets has no half-close",
                errno::EOPNOTSUPP,
            );
        }
        if s.listening {
            // Shutting a listener's reading side stops it listening.
            if rd {
                s.listening = false;
                let (queued, waiting) =
                    (core::mem::take(&mut s.pending), core::mem::take(&mut s.syn));
                for q in queued {
                    t.free(q, true);
                }
                t.refuse_waiting(waiting.into_iter());
            }
            return errno::ok(0);
        }
        let connected = s.connected || (s.proto == Proto::Dgram && s.remote.is_some());
        if !connected {
            return errno::fail(errno::ENOTCONN);
        }
        s.rd_shut |= rd;
        s.wr_shut |= wr;
        let peer = s.peer;
        // The peer reads end of file once it has what was already sent.
        if let Some(p) = peer.filter(|_| wr).and_then(|p| t.get_mut(p)) {
            p.eof = true;
        }
        errno::ok(0)
    })
}
