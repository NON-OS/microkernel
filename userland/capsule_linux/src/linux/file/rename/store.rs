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

//! A name moved in the store, with what the family keeps beside it: its
//! cached copy, its mode, its times and its extended attributes.

use crate::linux::abi::errno;

use super::super::meta::look;
use super::super::resolve::key;
use super::super::{cache, modes, store_name};

/// `from`, which is not a link, renamed to `to`.
pub(super) fn move_name(from: &[u8], to: &[u8]) -> u64 {
    if look(from).is_none() {
        return errno::fail(errno::ENOENT);
    }
    /* A file in the way is replaced, as rename(2) replaces it. */
    if let Some((_, false)) = look(to) {
        cache::forget(to);
        let _ = store_name::unlink(&key(to));
    }
    /* The store renames what it has: the family's copy goes in first. */
    if let Err(e) = cache::flush(from, true) {
        return errno::fail(e);
    }
    match store_name::rename(&key(from), &key(to)) {
        Ok(()) => {
            cache::renamed(from, to);
            modes::renamed(from, to);
            super::super::times::renamed(from, to);
            super::super::xattr_table::renamed(from, to);
            errno::ok(0)
        }
        Err(e) => errno::fail(super::super::store_err::errno_of(e)),
    }
}
