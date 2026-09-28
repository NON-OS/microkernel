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

/* Whether a name is a link, and readlinkat. */

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::super::super::at;
use super::super::node::{self};

/* Whether the name is itself a link. */
pub fn is_link(guest: &Guest, named: &[u8]) -> bool {
    let full = guest.links.follow(named.to_vec(), false);
    guest.links.target(&full).is_some()
}

pub fn readlinkat(guest: &Guest, dirfd: u64, path_ptr: u64, buf: u64, len: u64) -> u64 {
    if (len as i64) <= 0 {
        return errno::fail(errno::EINVAL);
    }
    let Some(full) = at::resolve_at(guest, dirfd, path_ptr) else {
        return errno::fail(errno::EFAULT);
    };
    let Some(to) = guest.links.target(&full) else {
        return match node::of(guest, full, false) {
            Ok(_) => errno::fail(errno::EINVAL),
            Err(e) => errno::fail(e),
        };
    };
    let n = to.len().min(len as usize);
    match guest.write(buf, &to[..n]) < n as i64 {
        true => errno::fail(errno::EFAULT),
        false => errno::ok(n as u64),
    }
}
