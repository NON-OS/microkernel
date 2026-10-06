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

//! `setsockopt`. Every option here is kept and reads back as Linux reads
//! it; one that would change nothing on the family's loopback is still kept,
//! since Linux keeps it too. One this capsule cannot honour is refused.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use crate::linux::net::fd::sock_of;
use crate::linux::net::sock;

pub fn setsockopt(guest: &Guest, fd: u64, level: u64, name: u64, val: u64, len: u64) -> u64 {
    let id = match sock_of(guest, fd) {
        Ok(id) => id,
        Err(e) => return e,
    };
    let Some(raw) = guest.read(val, (len as usize).min(16)) else {
        return errno::fail(errno::EFAULT);
    };
    if len < 4 {
        return errno::fail(errno::EINVAL);
    }
    sock::with(|t| match t.get_mut(id) {
        Some(s) => super::apply::apply(s, (level, name), &raw, len),
        None => errno::fail(errno::EBADF),
    })
}
