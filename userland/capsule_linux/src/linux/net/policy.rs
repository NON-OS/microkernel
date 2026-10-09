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

//! What this capsule lets a guest's sockets reach. A guest binds and
//! listens only on 127.0.0.0/8, which is the family's own: nothing outside
//! the capsule can connect to it. Anything else is EACCES, said by name.
//! A raw internet socket reaches below anything that could confine it and
//! is EPERM; a packet socket is a family socket.rs does not make, so it is
//! EAFNOSUPPORT. A stream whose chosen network is not
//! running is ENETUNREACH, with the network's name.

use alloc::format;

use crate::linux::abi::errno;
use crate::linux::say::say;

use super::sock::Addr;

/// EACCES for a bind or listen outside loopback, with the address.
pub fn not_loopback(call: &str, at: Addr) -> u64 {
    let [a, b, c, d] = at.ip;
    let line = format!(
        "[LINUX] refused {call} {a}.{b}.{c}.{d}:{}: a guest listens only on 127.0.0.0/8\n",
        at.port
    );
    say(line.as_bytes());
    errno::fail(errno::EACCES)
}

/// ENETUNREACH for a datagram to anywhere outside the family: the mixnet
/// carries streams, and a guest's datagrams have no other way out.
pub fn refuse_out(call: &str, to: Addr) -> u64 {
    let [a, b, c, d] = to.ip;
    let line = format!(
        "[LINUX] refused {call} {a}.{b}.{c}.{d}:{}: a guest's datagrams stay in the family\n",
        to.port
    );
    say(line.as_bytes());
    errno::fail(errno::ENETUNREACH)
}

/// ENETUNREACH for a stream whose chosen network is not running, with the
/// network by name (guest_route.rs). Nothing else is tried.
pub fn unreachable(to: Addr, why: &str) -> u64 {
    let [a, b, c, d] = to.ip;
    let line = format!("[LINUX] refused connect {a}.{b}.{c}.{d}:{}: {why}\n", to.port);
    say(line.as_bytes());
    errno::fail(errno::ENETUNREACH)
}

/// `errno` for a call this capsule declines, with why.
pub fn refuse(what: &str, errno: i64) -> u64 {
    say(format!("[LINUX] refused {what}\n").as_bytes());
    errno::fail(errno)
}
