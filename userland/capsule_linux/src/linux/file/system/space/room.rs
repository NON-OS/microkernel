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

/* The room a mount has. */

use super::super::super::resolve;
use super::super::declared::{PRIVATE, PRIVATE_NAMES};
use super::quota::names_free;
use super::used::{kept, under};

/* statfs's block size, the page size tmpfs counts in. */
pub const BSIZE: u64 = 4096;

/*
 * Blocks in all, blocks free, inodes in all and inodes free, as statfs
 * reports them.
 */
pub struct Room {
    pub blocks: u64,
    pub free: u64,
    pub files: u64,
    pub files_free: u64,
}

/* The room on the mount with options `opts`, mounted at `point`. */
pub fn of(point: &str, opts: &str) -> Result<Room, i64> {
    let none = Room { blocks: 0, free: 0, files: 0, files_free: 0 };
    match (point, opts.starts_with("rw")) {
        ("/", _) => {
            let (bytes, entries) = under(resolve::key(b"/").as_bytes())?;
            Ok(Room { blocks: bytes.div_ceil(BSIZE), free: 0, files: entries, files_free: 0 })
        }
        /* The private mounts share one quota, of bytes and of names. */
        (_, true) => {
            let now = kept()?;
            Ok(Room {
                blocks: PRIVATE / BSIZE,
                free: PRIVATE.saturating_sub(now.bytes) / BSIZE,
                files: PRIVATE_NAMES,
                files_free: names_free(now),
            })
        }
        _ => Ok(none),
    }
}
