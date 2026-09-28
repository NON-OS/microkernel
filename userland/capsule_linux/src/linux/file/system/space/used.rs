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

/* What the family keeps in its private directories, and its quota. */

use nonos_app_skeleton::clients::vfs;
use nonos_libc::mk_getpid;

use super::super::super::{cache, private};
use super::super::declared::PRIVATE;

/* The bytes the family keeps in its private directories. */
pub fn used() -> Result<u64, i64> {
    let stored = under(&private::root())?.0;
    Ok(stored.saturating_add_signed(cache::growth()))
}

/*
 * Whether what the family keeps, its unwritten copies counted, is within
 * its quota.
 */
pub fn within() -> Result<(), i64> {
    match used()? <= PRIVATE {
        true => Ok(()),
        false => Err(crate::linux::abi::errno::ENOSPC),
    }
}

/*
 * Bytes and entries under a store prefix. A walk the store cut short at
 * its node cap counts less than is there.
 */
pub(super) fn under(prefix: &[u8]) -> Result<(u64, u64), i64> {
    let (files, dirs, bytes, _) =
        vfs::dirstat(mk_getpid(), prefix).map_err(super::super::super::store_err::errno_of)?;
    Ok((bytes, u64::from(files) + u64::from(dirs)))
}
