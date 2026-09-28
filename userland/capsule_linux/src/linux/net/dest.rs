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

//! Where a send goes: nowhere but the peer for a stream, and for a datagram
//! the address or the Unix name it names, found now.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::peer_addr::To;
use super::sock::{Dest, Domain, Proto};

pub fn dest(guest: &Guest, proto: Proto, domain: Domain, to: Option<To>) -> Result<Dest, u64> {
    Ok(match (proto, to) {
        (Proto::Stream, _) | (_, None) => Dest::Default,
        (_, Some(To::Inet(a))) => Dest::Inet(a),
        (_, Some(To::Unix(_))) if domain == Domain::Inet => {
            return Err(errno::fail(errno::EAFNOSUPPORT))
        }
        (_, Some(To::Unix(ua))) => {
            let found = super::unix_name::resolve(guest, &ua)
                .ok_or(errno::EINVAL)
                .and_then(|name| super::unix_name::find(&name, Proto::Dgram));
            match found {
                Ok(t) => Dest::Sock(t),
                Err(e) => return Err(errno::fail(e)),
            }
        }
    })
}
