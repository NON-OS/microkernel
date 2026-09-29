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

/* The metadata a *at call names, after its flags. */

use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::super::super::{at, path};
use super::super::node::{self, Meta};
use super::super::statbuf::{build, STAT_LEN};
use super::calls::{AT_EMPTY_PATH, AT_SYMLINK_NOFOLLOW};

/*
 * The *at form's metadata: the name against the directory descriptor,
 * or the descriptor itself for an empty name with AT_EMPTY_PATH.
 */
pub fn meta_at(guest: &Guest, dirfd: u64, path_ptr: u64, flags: u64) -> Result<Meta, i64> {
    let name = path::read_path(guest, path_ptr).ok_or(errno::EFAULT)?;
    let dirfd = super::super::super::flags::dirfd(dirfd);
    if name.is_empty() {
        if flags & AT_EMPTY_PATH == 0 {
            return Err(errno::ENOENT);
        }
        if dirfd == super::super::super::flags::AT_FDCWD {
            return node::of(guest, guest.cwd.clone(), true);
        }
        let f = guest.fds.get(dirfd as usize).filter(|f| f.is_open()).ok_or(errno::EBADF)?;
        return node::of_fd(guest, f);
    }
    let named: Vec<u8> = at::named_at(guest, dirfd, &name)?;
    node::of(guest, named, flags & AT_SYMLINK_NOFOLLOW == 0)
}

pub(super) fn write_out(guest: &Guest, out: u64, m: &Meta) -> u64 {
    if guest.write(out, &build(m)) < STAT_LEN as i64 {
        return errno::fail(errno::EFAULT);
    }
    errno::ok(0)
}
