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

/*
 * Whether a hard link may join two names. Pure, so the host proofs hold it.
 *
 * Linux refuses a link from one mount to another with EXDEV, and the
 * mounts are the ones the family is told of (mounts::MOUNTS): the store's
 * tree at /, read-only, and a tmpfs at each private directory. A link here
 * is a copy, since the store has no inodes to share; one from the tree
 * into /tmp copied a file as large as anything installed into the
 * family's private directories, with nothing written and no quota asked.
 */

use crate::linux::abi::errno;
use crate::linux::file::mounts::of;

/* EXDEV unless `from` and `to` are on the same mount. */
pub fn same_mount(from: &[u8], to: &[u8]) -> Result<(), i64> {
    match of(from).0 == of(to).0 {
        true => Ok(()),
        false => Err(errno::EXDEV),
    }
}
