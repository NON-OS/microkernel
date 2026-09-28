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

//! `socketpair`: two connected Unix sockets, both ends in the family.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::fd::{install, SOCK_CLOEXEC, SOCK_NONBLOCK};
use super::sock::{self, Domain, Proto};
use super::sockaddr::{AF_INET, AF_UNIX};

const SOCK_STREAM: u64 = 1;
const SOCK_DGRAM: u64 = 2;
const TYPE_MASK: u64 = 0xF;

pub fn socketpair(guest: &mut Guest, family: u64, kind: u64, protocol: u64, out: u64) -> u64 {
    let flags = kind & (SOCK_NONBLOCK | SOCK_CLOEXEC);
    if kind & !(TYPE_MASK | flags) != 0 {
        return errno::fail(errno::EINVAL);
    }
    match family {
        f if f == u64::from(AF_UNIX) => {}
        // Linux has no connected pair for the internet families.
        f if f == u64::from(AF_INET) => return errno::fail(errno::EOPNOTSUPP),
        _ => return errno::fail(errno::EAFNOSUPPORT),
    }
    let proto = match kind & TYPE_MASK {
        SOCK_STREAM => Proto::Stream,
        SOCK_DGRAM => Proto::Dgram,
        _ => return errno::fail(errno::ESOCKTNOSUPPORT),
    };
    if protocol != 0 {
        return errno::fail(errno::EPROTONOSUPPORT);
    }
    let (a, b) = sock::with(|t| t.pair(Domain::Unix, proto, guest.pid));
    let fa = install(guest, a, flags);
    let Some(na) = errno::slot(fa) else {
        sock::with(|t| t.release(b, guest.pid));
        return fa;
    };
    let fb = install(guest, b, flags);
    let Some(nb) = errno::slot(fb) else {
        super::close::discard(guest, na as u64);
        return fb;
    };
    let mut pair = [0u8; 8];
    pair[..4].copy_from_slice(&(na as u32).to_le_bytes());
    pair[4..].copy_from_slice(&(nb as u32).to_le_bytes());
    // Linux copies the pair out before it installs either descriptor.
    if guest.write(out, &pair) < 8 {
        super::close::discard(guest, na as u64);
        super::close::discard(guest, nb as u64);
        return errno::fail(errno::EFAULT);
    }
    errno::ok(0)
}
