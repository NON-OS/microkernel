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

//! The SOCKS endpoint at 127.0.0.1:9050, read as it ships: SOCKS5 with no
//! authentication, SOCKS4 and SOCKS4a; CONNECT to an IPv4 address or a
//! name, the name kept a name; IPv6, BIND and UDP ASSOCIATE refused with
//! their own codes; every truncation "more to come"; a request bounded;
//! and no bytes a guest can send ever panic it.

mod fuzz;
mod replies;
mod talk;
mod talk_close;
mod v4;
mod v5;
mod v5_refused;

use crate::linux::net::routed::socks::{Connect, To};

/// SOCKS5's request for `to` on `port`.
pub(super) fn v5_request(cmd: u8, to: &To, port: u16) -> Vec<u8> {
    let mut out = vec![5, cmd, 0];
    match to {
        To::V4(ip) => out.extend_from_slice(&[&[1u8][..], ip].concat()),
        To::Name(n) => out.extend_from_slice(&[&[3, n.len() as u8][..], n].concat()),
    }
    out.extend_from_slice(&port.to_be_bytes());
    out
}

/// SOCKS4a's request for `name`, as Monero's `--proxy` client sends it:
/// address 0.0.0.1, an empty user id, the name.
pub(super) fn v4a_request(name: &[u8], port: u16) -> Vec<u8> {
    let mut out = vec![4, 1];
    out.extend_from_slice(&port.to_be_bytes());
    out.extend_from_slice(&[0, 0, 0, 1, 0]);
    out.extend_from_slice(name);
    out.push(0);
    out
}

pub(super) fn name(n: &str, port: u16) -> Connect {
    Connect { to: To::Name(n.as_bytes().to_vec()), port }
}

/// The next of a xorshift sequence: bytes no one chose.
pub(super) fn next(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}
