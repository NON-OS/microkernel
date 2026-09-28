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

//! The addresses the send and receive calls carry: the one a send names,
//! and the sender a receive writes back.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::flags::MSG_TRUNC;
use super::sock::Addr;
use super::sockaddr::{self, AF_INET};
use super::xfer_in::In;

/// The count a receive answers, with the sender written out. A stream has
/// no sender, and Linux says so with a length of zero.
pub fn finish(guest: &mut Guest, got: In, flags: u64, at: u64, alen: u64) -> u64 {
    if at != 0 {
        let wrote = match got.from {
            Some((domain, from)) => super::sockaddr_out::write(guest, at, alen, domain, from),
            None if alen != 0 && guest.write(alen, &0u32.to_le_bytes()) < 4 => {
                errno::fail(errno::EFAULT)
            }
            None => errno::ok(0),
        };
        if errno::slot(wrote).is_none() {
            return wrote;
        }
    }
    errno::ok(if flags & MSG_TRUNC != 0 { got.whole } else { got.n } as u64)
}

/// The address a send names, if any: IPv4 only.
pub fn address(guest: &Guest, at: u64, alen: u64) -> Result<Option<Addr>, u64> {
    if at == 0 {
        return Ok(None);
    }
    match sockaddr::read(guest, at, alen)? {
        (AF_INET, a) => Ok(Some(a)),
        _ => Err(errno::fail(errno::EAFNOSUPPORT)),
    }
}
