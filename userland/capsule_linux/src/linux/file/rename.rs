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

//! Moving a name.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::at::resolve_at;
use super::resolve::key;
use super::store_name;

pub fn rename(guest: &Guest, old: u64, new: u64) -> u64 {
    let (Some(from), Some(to)) = (
        resolve_at(guest, super::flags::AT_FDCWD, old),
        resolve_at(guest, super::flags::AT_FDCWD, new),
    ) else {
        return errno::fail(errno::EFAULT);
    };
    match store_name::rename(&key(&from), &key(&to)) {
        Ok(()) => errno::ok(0),
        Err(_) => errno::fail(errno::ENOENT),
    }
}

const RENAME_NOREPLACE: u64 = 1;

/// `renameat` and `renameat2`. NOREPLACE refuses an existing target;
/// EXCHANGE would need two names swapped at once, which the store cannot
/// do, so it is refused rather than done as two renames that could half-fail.
pub fn renameat2(guest: &Guest, olddir: u64, old: u64, newdir: u64, new: u64, flags: u64) -> u64 {
    if flags & !RENAME_NOREPLACE != 0 {
        return errno::fail(errno::EINVAL);
    }
    let (Some(from), Some(to)) = (resolve_at(guest, olddir, old), resolve_at(guest, newdir, new))
    else {
        return errno::fail(errno::EFAULT);
    };
    if flags & RENAME_NOREPLACE != 0 && super::meta::stat::look(&to).is_some() {
        return errno::fail(errno::EEXIST);
    }
    match store_name::rename(&key(&from), &key(&to)) {
        Ok(()) => errno::ok(0),
        Err(_) => errno::fail(errno::ENOENT),
    }
}
