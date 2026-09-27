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

//! `getcwd`, `access` and `readlink`: the three questions a program asks
//! about a path without opening it.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::super::{path, resolve};
use super::stat;

pub fn access(guest: &Guest, path_ptr: u64) -> u64 {
    let Some(name) = path::read_path(guest, path_ptr) else {
        return errno::fail(errno::EFAULT);
    };
    let full = guest.links.follow(resolve::visible(&guest.cwd, &name), true);
    match stat::look(&full) {
        Some(_) => errno::ok(0),
        None => errno::fail(errno::ENOENT),
    }
}

/// A link's target, from the image's table. A path that exists and is not a
/// link is EINVAL, as Linux answers.
pub fn readlink(guest: &Guest, path_ptr: u64, buf: u64, len: u64) -> u64 {
    let Some(name) = path::read_path(guest, path_ptr) else {
        return errno::fail(errno::EFAULT);
    };
    let full = guest.links.follow(resolve::visible(&guest.cwd, &name), false);
    if let Some(to) = guest.links.target(&full) {
        let n = to.len().min(len as usize);
        return match guest.write(buf, &to[..n]) < n as i64 {
            true => errno::fail(errno::EFAULT),
            false => errno::ok(n as u64),
        };
    }
    match stat::look(&full) {
        Some(_) => errno::fail(errno::EINVAL),
        None => errno::fail(errno::ENOENT),
    }
}
