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

/* What the family keeps in its scratch, its unwritten copies added. */
pub fn kept() -> Result<Kept, i64> {
    Ok(copies(under(&private::root())?))
}

/*
 * Under a root, every unwritten copy added: a copy counts against each
 * quota until it is in the store, so none is passed while it is open.
 */
pub(super) fn copies((bytes, names): (u64, u64)) -> Kept {
    Kept {
        bytes: bytes.saturating_add_signed(cache::growth()),
        names: names.saturating_add(cache::unstored()),
    }
}

/* Whether what is kept where `at` is, copies counted, is within its quota. */
pub fn within(at: &[u8]) -> Result<(), i64> {
    room(at, Kept { bytes: 0, names: 0 }, true)
}

/*
 * Whether `more` may be added where `at` is: a new name, or a name and the
 * bytes a hard link copies. The home (home_room.rs) has its own quota.
 */
pub fn room_for(at: &[u8], more: Kept) -> Result<(), i64> {
    room(at, more, false)
}

/* `whole`: the copies already hold `more`; otherwise it is added. */
fn room(at: &[u8], more: Kept, whole: bool) -> Result<(), i64> {
    if let Some(home) = private::in_home(at).then(private::home_root).flatten() {
        return super::home_room::room(&home, more, whole);
    }
    let now = kept()?;
    match whole {
        true => allows(more, now),
        false => allows(now, now.plus(more)),
    }
}

/*
 * Bytes and entries under a store prefix. A walk the store cut short at
 * its node cap counts less than is there; the names quotas keep it below.
 */
pub(super) fn under(prefix: &[u8]) -> Result<(u64, u64), i64> {
    let (files, dirs, bytes, _) =
        vfs::dirstat(mk_getpid(), prefix).map_err(super::super::super::store_err::errno_of)?;
    Ok((bytes, u64::from(files) + u64::from(dirs)))
}
