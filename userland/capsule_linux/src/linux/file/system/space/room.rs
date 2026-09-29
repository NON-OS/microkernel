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
use super::super::declared::PRIVATE;
use super::used::{under, used};

/* statfs's block size, the page size tmpfs counts in. */
pub const BSIZE: u64 = 4096;

/* Blocks in all, blocks free, and inodes in all, as statfs reports them. */
pub struct Room {
    pub blocks: u64,
    pub free: u64,
    pub files: u64,
}

/* The room on the mount with options `opts`, mounted at `point`. */
pub fn of(point: &str, opts: &str) -> Result<Room, i64> {
    let none = Room { blocks: 0, free: 0, files: 0 };
    match (point, opts.starts_with("rw")) {
        ("/", _) => {
            let (bytes, entries) = under(resolve::key(b"/").as_bytes())?;
            Ok(Room { blocks: bytes.div_ceil(BSIZE), free: 0, files: entries })
        }
        /* No inode limit is kept, which Linux says with no inodes at all. */
        (_, true) => Ok(Room {
            blocks: PRIVATE / BSIZE,
            free: PRIVATE.saturating_sub(used()?) / BSIZE,
            files: 0,
        }),
        _ => Ok(none),
    }
}
