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

//! Bytes taken from a stream through net.anon, and what it is ready for.

use alloc::vec::Vec;

use super::super::anon_answer::got;
use super::super::anon_call::call;
use super::super::anon_ops::{OP_RECV, PAYLOAD_MAX};
use super::super::sock::{self, Backend};
use crate::linux::abi::errno::{self, EBADF};

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
