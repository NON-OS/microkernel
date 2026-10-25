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
//! `recvmsg` on the resolver, as musl's resolver reads its answers: the
//! oldest answer into the first buffer the message names, the nameserver's
//! address as its sender, and no control data. An answer is one datagram, so
//! it fills one buffer, as a datagram socket's read does; past that buffer's
//! length it is cut and MSG_TRUNC is said.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::super::flags::MSG_TRUNC;
use super::hdr::{hdr, CONTROLLEN_AT, FLAGS_AT, NAMELEN_AT};

pub fn recvmsg(guest: &mut Guest, fd: u64, msg: u64) -> u64 {
    let h = match hdr(guest, msg) {
        Ok(h) => h,
        Err(e) => return e,
    };
    let (base, len) = h.iov.iter().copied().find(|(_, l)| *l > 0).unwrap_or((0, 0));
    let want = super::super::dns::waiting(guest, fd);
    let (name, lenp) = if h.name != 0 { (h.name, msg + NAMELEN_AT) } else { (0, 0) };
    let value = super::super::resolver::answer(guest, fd, base, len, name, lenp);
    if errno::slot(value).is_none() {
        return value;
    }
    let cut = if want.is_some_and(|w| w > len) { MSG_TRUNC as u32 } else { 0 };
    if guest.write(msg + CONTROLLEN_AT, &0u64.to_le_bytes()) < 8
        || guest.write(msg + FLAGS_AT, &cut.to_le_bytes()) < 4
    {
        return errno::fail(errno::EFAULT);
    }
    value
}
