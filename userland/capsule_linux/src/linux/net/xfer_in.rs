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

//! Bytes into a guest from a family socket, or a stream net.sockets or
//! net.anon holds.

use alloc::vec;

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::flags::{MSG_OOB, MSG_PEEK, MSG_TRUNC};
use super::iov::{self, Iov};
use super::sock::{self, Backend, Peer, Proto, Via};

pub struct In {
    /// Bytes put in the guest's buffers.
    pub n: usize,
    /// A datagram's length before it was cut to fit.
    pub whole: usize,
    /// Where a datagram came from; a stream, and an unnamed sender, say
    /// nothing.
    pub from: Option<Peer>,
}

/// Receive into `iov` from socket `id`, `skip` bytes of it filled already.
pub fn recv(guest: &Guest, id: u32, iov: &Iov, skip: usize, flags: u64) -> Result<In, u64> {
    if flags & MSG_OOB != 0 {
        return Err(errno::fail(errno::EINVAL));
    }
    let Some((proto, svc)) =
        sock::with(|t| t.get(id).map(|s| (s.proto, s.svc.as_ref().map(Backend::via))))
    else {
        return Err(errno::fail(errno::EBADF));
    };
    let want = iov::total(iov).saturating_sub(skip);
    let peek = flags & MSG_PEEK != 0;
    if let Some(Via::Sockets(h)) = svc {
        let bytes = super::stream::recv_bytes(h, want)?;
        iov::scatter(guest, iov, skip, &bytes)?;
        return Ok(In { n: bytes.len(), whole: bytes.len(), from: None });
    }
    if let Some(Via::Anon) = svc {
        let bytes = super::anon_stream::take(id, want, peek)?;
        /* A stream's MSG_TRUNC discards what it would have read. */
        if flags & MSG_TRUNC == 0 {
            iov::scatter(guest, iov, skip, &bytes)?;
        }
        return Ok(In { n: bytes.len(), whole: bytes.len(), from: None });
    }
    let (bytes, whole, from) = sock::with(|t| t.take(id, want, peek)).map_err(errno::fail)?;
    /* A stream's MSG_TRUNC discards what it would have read. */
    if proto != Proto::Stream || flags & MSG_TRUNC == 0 {
        iov::scatter(guest, iov, skip, &bytes)?;
    }
    Ok(In { n: bytes.len(), whole, from })
}

/// `read` on a socket.
pub fn read(guest: &Guest, id: u32, buf: u64, len: u64) -> u64 {
    match recv(guest, id, &vec![(buf, len)], 0, 0) {
        Ok(got) => errno::ok(got.n as u64),
        Err(e) => e,
    }
}
