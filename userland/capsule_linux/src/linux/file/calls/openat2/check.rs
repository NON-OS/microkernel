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

/* The name walked as open_how's rules allow. */

use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::super::super::{at, mounts, path, walk};
use super::how::open_how;
use super::open::{BENEATH, IN_ROOT};
use super::rooted::rooted;
use super::walked::walked;

pub(super) fn check(
    guest: &Guest,
    dirfd: u64,
    path_ptr: u64,
    how: u64,
    size: u64,
) -> Result<(Vec<u8>, u64, u64), i64> {
    let (flags, mode, rules) = open_how(guest, how, size)?;
    let name = path::name_of(guest, path_ptr)?;
    let base = walk::follow(guest, at::named_at(guest, dirfd, b".")?, true);
    if rules & BENEATH != 0 && name.first() == Some(&b'/') {
        return Err(errno::EXDEV);
    }
    if rules & IN_ROOT != 0 {
        let mount = mounts::of(&base).0;
        let inside = walk::walk_under(guest, &base, name, true, |s| rooted(s, rules, mount))?;
        return Ok((inside, flags, mode));
    }
    let named = at::named_at(guest, dirfd, &name)?;
    walked(guest, &base, &named, rules)?;
    Ok((named, flags, mode))
}
