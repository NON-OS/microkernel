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
 * A stored file's bytes taken whole into the family's copy, or refused. A
 * family holds at most MAX_FILE bytes of a file it writes, and a flush puts
 * its copy over the whole file, so a longer file held cut short would lose
 * its tail at the first write. The read asks one byte past the most
 * (linux/whole.rs), and a file that fills it is refused, EFBIG, and stays
 * in the store as it was.
 */

use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::whole::{ask, whole};

use super::super::super::root::Key;
use super::super::super::store;
use super::table::{now, MAX_FILE};
use super::too_large::too_large;

/* The bytes of the file at `key` and when it last changed. */
pub(super) fn stored(key: &Key) -> Result<(Vec<u8>, u64), i64> {
    let at = store::stat_full(key).map_or_else(|_| now(), |s| s.2);
    let data = store::read(key, ask(MAX_FILE as u32)).map_err(|_| errno::EIO)?;
    Ok((whole(data, MAX_FILE as u32).ok_or_else(too_large)?, at))
}
