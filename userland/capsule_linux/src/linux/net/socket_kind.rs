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

//! Which kind of socket a `socket` type asks for, and the protocol that is
//! its own.

use crate::linux::abi::errno;

use super::policy::refuse;
use super::sock::{Domain, Proto};

const SOCK_STREAM: u64 = 1;
const SOCK_DGRAM: u64 = 2;
const SOCK_RAW: u64 = 3;
const SOCK_SEQPACKET: u64 = 5;
pub const TYPE_MASK: u64 = 0xF;
const IPPROTO_TCP: u64 = 6;
const IPPROTO_UDP: u64 = 17;
/// The one protocol a Unix socket takes besides 0.
const PF_UNIX: u64 = 1;

/// The socket `kind` (its type bits) makes in `domain`, and its own
/// protocol; the errno answer when there is none.
pub fn kind(kind: u64, domain: Domain) -> Result<(Proto, u64), u64> {
    match (kind & TYPE_MASK, domain) {
        (SOCK_STREAM, Domain::Inet) => Ok((Proto::Stream, IPPROTO_TCP)),
        (SOCK_DGRAM, Domain::Inet) => Ok((Proto::Dgram, IPPROTO_UDP)),
        (SOCK_RAW, Domain::Inet) => {
            crate::linux::boxed::note(crate::linux::boxed::Kind::Network, b"SOCK_RAW");
            Err(refuse("SOCK_RAW: raw sockets reach below any confinement", errno::EPERM))
        }
        (SOCK_STREAM, Domain::Unix) => Ok((Proto::Stream, PF_UNIX)),
        /* Linux gives a raw Unix socket datagram semantics. */
        (SOCK_DGRAM | SOCK_RAW, Domain::Unix) => Ok((Proto::Dgram, PF_UNIX)),
        (SOCK_SEQPACKET, Domain::Unix) => Err(refuse(
            "SOCK_SEQPACKET: a Unix stream here does not keep message boundaries",
            errno::ESOCKTNOSUPPORT,
        )),
        _ => Err(errno::fail(errno::ESOCKTNOSUPPORT)),
    }
}
