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
use super::quota::{allows, Kept};

/*
 * What the family keeps in its private directories: the bytes and names
 * the store has under its root, with what the family's unwritten copies
 * add to the bytes, and the files they hold that the store has no name
 * for yet added to the names.
 */
pub fn kept() -> Result<Kept, i64> {
    let (bytes, names) = under(&private::root())?;
    Ok(Kept {
        bytes: bytes.saturating_add_signed(cache::growth()),
        names: names.saturating_add(cache::unstored()),
    })
}

/*
 * Whether what the family keeps, its unwritten copies counted, is within
 * its quota: asked before a copy grows the store or gives it a new name.
 */
pub fn within() -> Result<(), i64> {
    allows(Kept { bytes: 0, names: 0 }, kept()?)
}

/*
 * Whether `more` may be added to what the family keeps: a new name, or a
 * name and the bytes a hard link copies under it.
 */
pub fn room_for(more: Kept) -> Result<(), i64> {
    let now = kept()?;
    allows(now, now.plus(more))
}

/*
 * Bytes and entries under a store prefix. A walk the store cut short at
 * its node cap counts less than is there; under a private root the names
 * quota keeps the count far below that cap.
 */
pub(super) fn under(prefix: &[u8]) -> Result<(u64, u64), i64> {
    let (files, dirs, bytes, _) =
        vfs::dirstat(mk_getpid(), prefix).map_err(super::super::super::store_err::errno_of)?;
    Ok((bytes, u64::from(files) + u64::from(dirs)))
}
