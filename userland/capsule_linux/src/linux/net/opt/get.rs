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

//! `getsockopt`, for the options `opt_set` keeps and the ones a socket
//! reports about itself.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use crate::linux::net::fd::sock_of;
use crate::linux::net::sock;

pub fn getsockopt(guest: &Guest, fd: u64, level: u64, name: u64, val: u64, lenp: u64) -> u64 {
    let id = match sock_of(guest, fd) {
        Ok(id) => id,
        Err(e) => return e,
    };
    let Some(raw) = guest.read(lenp, 4) else {
        return errno::fail(errno::EFAULT);
    };
    let room = i32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]);
    if room < 0 {
        return errno::fail(errno::EINVAL);
    }
    let value = sock::with(|t| t.get_mut(id).map(|s| super::value::value(s, level, name)));
    let bytes = match value {
        Some(Ok(bytes)) => bytes,
        Some(Err(e)) => return e,
        None => return errno::fail(errno::EBADF),
    };
    let n = bytes.len().min(room as usize);
    if guest.write(val, &bytes[..n]) < n as i64 || guest.write(lenp, &(n as u32).to_le_bytes()) < 4
    {
        return errno::fail(errno::EFAULT);
    }
    errno::ok(0)
}

/// ENOPROTOOPT for an option this capsule does not keep, said by name: a
/// program may depend on it, and Linux would have kept it.
pub fn unknown(call: &str, level: u64, name: u64) -> u64 {
    let what = alloc::format!("{call} level {level} option {name}: not kept for a guest socket");
    crate::linux::net::policy::refuse(&what, errno::ENOPROTOOPT)
}
