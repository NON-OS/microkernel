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

//! Bytes on a stream that reaches outside the family through net.anon: the
//! calls, around the socket entry's own state (sock/anon_rx.rs), which is
//! read and changed between them, never while a call is out.

use alloc::vec::Vec;

use crate::linux::abi::errno::{self, EBADF, EIO, EPIPE};

use super::anon_answer::{got, sent};
use super::anon_call::call;
use super::anon_ops::{OP_CLOSE_STREAM, OP_RECV, OP_SEND, PAYLOAD_MAX};
use super::sock::{self, Backend, Close};

/// The most one send carries: net.anon's body, less the stream id.
pub const SEND_MAX: usize = PAYLOAD_MAX - 2;

pub fn close(c: Close) {
    let _ = call(c.port, OP_CLOSE_STREAM, &c.id.to_le_bytes(), 0);
}

/// Send `bytes` on socket `id`'s stream: the count net.anon took, or an
/// errno.
pub fn send(id: u32, bytes: &[u8]) -> u64 {
    let target = sock::with(|t| {
        let s = t.get(id)?;
        let a = s.svc.as_ref()?.anon()?;
        Some((a.port, a.id.filter(|_| !s.wr_shut && !s.broken)))
    });
    let Some((port, sid)) = target else {
        return errno::fail(EBADF);
    };
    let Some(sid) = sid else {
        return errno::fail(EPIPE);
    };
    let mut body = Vec::with_capacity(2 + bytes.len());
    body.extend_from_slice(&sid.to_le_bytes());
    body.extend_from_slice(bytes);
    let reply = call(port, OP_SEND, &body, 4);
    match sent(reply.as_ref().map(|(st, b)| (*st, b.as_slice())), bytes.len()) {
        Ok(n) => errno::ok(n as u64),
        Err(e) => {
            /* Whether those bytes went is not known, so nothing may follow them. */
            if e == EIO {
                sock::with(|t| t.get_mut(id).map(|s| s.broken = true));
            }
            errno::fail(e)
        }
    }
}

/// Take what has arrived on socket `id`'s stream into its buffer, when the
/// buffer is empty and the stream may still bring something.
fn fill(id: u32) {
    let ask = sock::with(|t| {
        let s = t.get(id).filter(|s| !s.rd_shut)?;
        let a = s.svc.as_ref()?.anon().filter(|a| a.wants_fill())?;
        Some((a.port, a.id?))
    });
    let Some((port, sid)) = ask else {
        return;
    };
    let reply = call(port, OP_RECV, &sid.to_le_bytes(), PAYLOAD_MAX);
    let got = got(reply.as_ref().map(|(st, b)| (*st, b.as_slice())));
    sock::with(|t| {
        let a = t.get_mut(id).and_then(|s| s.svc.as_mut()).and_then(Backend::anon_mut);
        if let Some(a) = a.filter(|a| a.id == Some(sid)) {
            a.fill(got);
        }
    });
}

/// Up to `want` bytes from socket `id`'s stream, or its errno.
pub fn take(id: u32, want: usize, peek: bool) -> Result<Vec<u8>, u64> {
    if want != 0 {
        fill(id);
    }
    sock::with(|t| {
        let s = t.get_mut(id).ok_or(EBADF)?;
        let shut = s.rd_shut;
        let a = s.svc.as_mut().and_then(Backend::anon_mut).ok_or(EBADF)?;
        a.take(want, peek, shut)
    })
    .map_err(errno::fail)
}

/// poll's bits for socket `id`'s stream, after taking in what has arrived.
pub fn bits(id: u32) -> Option<u16> {
    fill(id);
    sock::with(|t| {
        let s = t.get(id)?;
        Some(s.svc.as_ref()?.anon()?.bits(s.rd_shut, s.wr_shut))
    })
}
