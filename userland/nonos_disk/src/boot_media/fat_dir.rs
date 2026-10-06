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
 * One name looked up in one FAT directory, by its 8.3 short name, which
 * every FAT writer gives an entry whether or not it also writes a long
 * one. A directory is read one cluster at a time along its chain, and at
 * most `MAX_CLUSTERS` of them: an ESP's directories are a handful of
 * entries, and a looping chain must not hold the installer.
 */

use alloc::vec::Vec;

use super::fat::Fat;
use super::read::{le16, le32, sectors};
use crate::sink::BlockSink;

const MAX_CLUSTERS: usize = 64;

pub struct Entry {
    pub cluster: u32,
    pub size: u64,
    pub dir: bool,
}

/* `dir` zero is the root. `name` is the 11-byte space-padded short name. */
pub fn find(fat: &Fat, disk: &mut dyn BlockSink, dir: u32, name: &[u8; 11]) -> Option<Entry> {
    if dir == 0 && fat.bits != 32 {
        return scan(&sectors(disk, fat.root_lba, fat.root_sectors)?, name)?;
    }
    let mut cluster = if dir == 0 { fat.root_cluster } else { dir };
    for _ in 0..MAX_CLUSTERS {
        let bytes: Vec<u8> = sectors(disk, fat.cluster_lba(cluster), fat.per_cluster)?;
        if let Some(found) = scan(&bytes, name) {
            return found;
        }
        cluster = fat.next(disk, cluster)?;
    }
    None
}

/* `Some(None)` at the end-of-directory mark; `None` to read on. */
fn scan(bytes: &[u8], name: &[u8; 11]) -> Option<Option<Entry>> {
    for e in bytes.chunks_exact(32) {
        let attr = e[11];
        if e[0] == 0 {
            return Some(None);
        }
        if e[0] == 0xE5 || attr & 0x0F == 0x0F || attr & 0x08 != 0 {
            continue;
        }
        if e[..11].eq_ignore_ascii_case(name) {
            let cluster = (le16(e, 20) << 16 | le16(e, 26)) as u32;
            return Some(Some(Entry { cluster, size: le32(e, 28), dir: attr & 0x10 != 0 }));
        }
    }
    None
}
