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

//! Bytes sent on a stream through net.anon, and the stream closed.

use alloc::vec::Vec;

use super::super::anon_answer::sent;
use super::super::anon_call::call;
use super::super::anon_ops::{OP_CLOSE_STREAM, OP_SEND, PAYLOAD_MAX};
use super::super::sock::{self, Close};
use crate::linux::abi::errno::{self, EBADF, EIO, EPIPE};

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
