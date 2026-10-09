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
 * The models this personality vouches for, by tier: a name, its length and
 * the SHA-256 the file must have. The tables are part of the signed
 * personality, so its measurement covers every digest in them; the disk
 * only says where the bytes wait. Every file is one the Qwen team
 * publishes in its GGUF repositories on Hugging Face, each digest the one
 * its LFS pointer names. The tables sit beside this file, one a family,
 * and nothing is pinned anywhere else.
 */

use super::pinned_coder::CODER;
use super::pinned_qwen25::QWEN25;
use super::pinned_qwen25_big::QWEN25_BIG;
use super::pinned_qwen3::QWEN3;

pub struct Pinned {
    pub tier: &'static str,
    pub name: &'static [u8],
    pub bytes: u64,
    pub sha256: [u8; 32],
}

/* Every table, smallest family first. */
const FAMILIES: &[&[Pinned]] = &[QWEN25, QWEN25_BIG, QWEN3, CODER];

/*
 * The tables as the code reads them. A static, not the const: a const is
 * copied into each place that names it, and two copies of a table hand out
 * two addresses for one pin, so an entry found by name would not be the
 * entry the list holds. The const is named only here and in the build-time
 * check below.
 */
static TABLES: &[&[Pinned]] = FAMILIES;

/* Every pinned file, family by family, in table order. */
pub fn all() -> impl Iterator<Item = &'static Pinned> {
    TABLES.iter().flat_map(|f| f.iter())
}

pub fn pin_of(name: &[u8]) -> Option<&'static Pinned> {
    all().find(|p| p.name == name)
}

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
