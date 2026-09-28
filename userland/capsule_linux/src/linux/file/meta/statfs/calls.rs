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

/* statfs and fstatfs. */

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

use super::super::super::{at, walk};
use super::fill::fill;

pub(super) const BSIZE: u64 = 1024;

pub(super) const BLOCKS: u64 = 1 << 20;

pub(super) const FREE: u64 = BLOCKS / 2;

pub(super) const ST_RDONLY: u64 = 1;

pub fn statfs(guest: &Guest, path: u64, out: u64) -> u64 {
    let Some(named) = at::resolve_at(guest, super::super::super::flags::AT_FDCWD, path) else {
        return errno::fail(errno::EFAULT);
    };
    let full = walk::follow(guest, named, true);
    if super::super::stat::look(&full).is_none() {
        return errno::fail(errno::ENOENT);
    }
    fill(guest, &full, out)
}

pub fn fstatfs(guest: &Guest, fd: u64, out: u64) -> u64 {
    match guest.fds.get(fd as usize).filter(|f| f.is_open()) {
        Some(f) if matches!(f.kind, Kind::File | Kind::Dir) => fill(guest, &f.path.clone(), out),
        /* What has no path is on no mount a guest can name: the root's. */
        Some(_) => fill(guest, b"/", out),
        None => errno::fail(errno::EBADF),
    }
}
