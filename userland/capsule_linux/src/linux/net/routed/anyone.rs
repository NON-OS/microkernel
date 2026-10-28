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

//! One try at a stream over the Anyone network, through net.anon's handle
//! front. The host goes as written, a name or a dotted quad: the exit
//! resolves a name, so nothing on this machine looks one up. net.anon
//! answers an open as soon as its BEGIN is sent, so a far end that refuses
//! is heard by the first read (sock/anon_rx.rs).

use super::super::anon_answer::{open_body, opened};
use super::super::anon_call::call;
use super::super::anon_ops::OP_OPEN_STREAM;
use super::entry::Dest;
use super::retry::{not_ready, refused};
use crate::linux::abi::errno::EINVAL;

pub enum Tried {
    /// Open, as net.anon's stream `id`.
    Opened(u16),
    /// Its network has not come up yet, for this reason: worth asking again.
    NotReady(&'static str),
    /// Not open, with this errno and reason, and not to be asked again.
    Refused(i64, &'static str),
}

pub fn try_open(port: u32, dest: &Dest) -> Tried {
    let Some(body) = open_body(&dest.host, dest.port) else {
        return Tried::Refused(EINVAL, "the host cannot be named to net.anon");
    };
    let reply = call(port, OP_OPEN_STREAM, &body, 2);
    let status = reply.as_ref().map(|(st, _)| *st);
    match opened(reply.as_ref().map(|(st, b)| (*st, b.as_slice()))) {
        Ok(id) => Tried::Opened(id),
        Err(e) => match status.and_then(not_ready) {
            Some(why) => Tried::NotReady(why),
            None => Tried::Refused(e, refused(status)),
        },
    }
}
