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

/* Moving a name. */

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::at::resolve_at;
use super::meta::look;
use super::resolve::key;
use super::{cache, modes, store_name, synth};

const RENAME_NOREPLACE: u64 = 1;

pub fn renameat2(guest: &Guest, olddir: u64, old: u64, newdir: u64, new: u64, flags: u64) -> u64 {
    if flags & !RENAME_NOREPLACE != 0 {
        return errno::fail(errno::EINVAL);
    }
    let (Some(from), Some(to)) = (resolve_at(guest, olddir, old), resolve_at(guest, newdir, new))
    else {
        return errno::fail(errno::EFAULT);
    };
    let there = look(&to).is_some() || guest.links.target(&to).is_some();
    if flags & RENAME_NOREPLACE != 0 && there {
        return errno::fail(errno::EEXIST);
    }
    if [&from, &to].iter().any(|p| synth::owns(p) || key(p).writable().is_err()) {
        return errno::fail(errno::EROFS);
    }
    if from == to {
        return errno::ok(0);
    }
    if guest.links.rename(&from, to.clone()) {
        return errno::ok(0);
    }
    if look(&from).is_none() {
        return errno::fail(errno::ENOENT);
    }
    /* A file in the way is replaced, as rename(2) replaces it. */
    if let Some((_, false)) = look(&to) {
        cache::forget(&to);
        let _ = store_name::unlink(&key(&to));
    }
    /* The store renames what it has: the family's copy goes in first. */
    if let Err(e) = cache::flush(&from, true) {
        return errno::fail(e);
    }
    match store_name::rename(&key(&from), &key(&to)) {
        Ok(()) => {
            cache::renamed(&from, &to);
            modes::renamed(&from, &to);
            super::times::renamed(&from, &to);
            super::xattr_table::renamed(&from, &to);
            errno::ok(0)
        }
        Err(e) => errno::fail(super::store_err::errno_of(e)),
    }
}
