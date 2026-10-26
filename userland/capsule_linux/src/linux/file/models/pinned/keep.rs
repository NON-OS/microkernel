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

use super::{Pinned, FAMILIES};

/*
 * What the data volume can keep. A directory entry holds 56 bytes of name
 * (src/fs/blockfs/dir_consts.rs, NAME_BYTES), and beside every file the
 * import path writes `<name>.sha256`, its record, and while a download is
 * coming `<name>.partial`, its mark (src/fs/blockfs_volume/import_guard/
 * name.rs). The kernel refuses to stream a name longer than 49 bytes with
 * its slash, the most that leaves room for the mark
 * (src/fs/blockfs_volume/import_feed/live.rs, NAME_MAX = 1 + 56 - 8). A
 * pin under a longer name could be listed but never fetched or imported.
 */
pub const ENTRY_BYTES: usize = 56;
pub const STREAM_NAME_MAX: usize = 1 + ENTRY_BYTES - b".partial".len();

/* Whether the volume can keep `name` ("/file"), its record and its mark. */
pub const fn keepable(name: &[u8]) -> bool {
    if name.len() < 2 || name[0] != b'/' {
        return false;
    }
    let file = name.len() - 1;
    name.len() <= STREAM_NAME_MAX
        && file + b".sha256".len() <= ENTRY_BYTES
        && file + b".partial".len() <= ENTRY_BYTES
}

const fn every_one_keepable(tables: &[&[Pinned]]) -> bool {
    let mut t = 0;
    while t < tables.len() {
        let mut i = 0;
        while i < tables[t].len() {
            if !keepable(tables[t][i].name) {
                return false;
            }
            i += 1;
        }
        t += 1;
    }
    true
}

/* The build stops on a pin the volume could not keep, so none ships. */
const _: () = assert!(
    every_one_keepable(FAMILIES),
    "a pinned model name is too long for the data volume to keep"
);
