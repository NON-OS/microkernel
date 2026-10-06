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

//! A stream outside the family over the Anyone network, when that is the
//! network chosen (guest_route.rs). net.anon's handle front opens it; the
//! family's entry keeps the stream's id.
//!
//! An address this capsule invented for a name goes as the name, and any
//! other as its dotted quad: the exit resolves a name, so nothing on this
//! machine looks one up. net.anon answers an open as soon as its BEGIN is
//! sent, so a far end that refuses is heard by the first read, which says
//! ECONNREFUSED as Linux does after a refused connect (sock/anon_rx.rs).

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::anon_answer::{dotted, open_body, opened};
use super::anon_call::call;
use super::anon_ops::OP_OPEN_STREAM;
use super::dns::host_for;
use super::sock::{self, Addr};

pub fn connect(guest: &Guest, id: u32, to: Addr, port: u32) -> u64 {
    let host = host_for(guest, to.ip).unwrap_or_else(|| dotted(to.ip));
    let Some(body) = open_body(&host, to.port) else {
        return errno::fail(errno::EINVAL);
    };
    let reply = call(port, OP_OPEN_STREAM, &body, 2);
    let sid = match opened(reply.as_ref().map(|(st, b)| (*st, b.as_slice()))) {
        Ok(sid) => sid,
        Err(e) => return errno::fail(e),
    };
    match sock::with(|t| t.adopt_anon(id, port, sid, to)) {
        Ok(()) => errno::ok(0),
        Err(e) => errno::fail(e),
    }
}
