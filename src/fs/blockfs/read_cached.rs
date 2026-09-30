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

//! Reading a file by range with its state kept between reads, so a file
//! read in pieces fetches each sealed sector about once. The cache's lock
//! is its own, since reads share the volume's; it is only tried, and held
//! only to take the state out or put it back, never across a disk read.

use spin::Mutex;

use super::file_cache::{FileCache, FileId, Held};
use super::index_block::Index;
use super::read_file::read_index;
use super::read_file_flat::read_flat;
use super::tree_range::read_range;
use super::tree_reader::TreeReader;
use super::tree_sealed::{fault, SealedSource};
use super::{BlockFsError, BlockFsMount, BlockFsNode};
use crate::fs::cryptoblock::ReadAhead;

/// One volume's cache: the state of the last file read on it.
pub struct ReadCache(Mutex<FileCache<ReadAhead>>);

impl ReadCache {
    pub const fn new() -> Self {
        ReadCache(Mutex::new(FileCache::new()))
    }

    /// `read_file_at` for the file whose node, `node`, is at `node_lba` on
    /// the volume `mount` describes. The caller holds the volume's lock.
    pub fn read_at(
        &self,
        key: &[u8; 32],
        mount: &BlockFsMount,
        node_lba: u64,
        node: &BlockFsNode,
        offset: u64,
        out: &mut [u8],
    ) -> Result<usize, BlockFsError> {
        if node.first_record_lba == 0 || node.size == 0 {
            return Ok(0);
        }
        let (volume, generation) = (mount.superblock.uuid, mount.superblock.generation);
        let (index_lba, size) = (node.first_record_lba, node.size);
        let epoch = crate::fs::cryptoblock::epoch();
        let id = FileId { volume, generation, epoch, node_lba, index_lba, size };
        let kept = self.0.try_lock().and_then(|mut cache| cache.take(&id));
        let mut held = match kept {
            Some(held) => held,
            None => match read_index(key, node)? {
                Index::Flat(index) => return read_flat(key, node, &index, offset, out),
                Index::Tree(root) => {
                    Held { reader: TreeReader::new(root), ahead: ReadAhead::new() }
                }
            },
        };
        let mut source = SealedSource { key, ahead: &mut held.ahead };
        let got = read_range(&mut source, &mut held.reader, node.size, offset, out).map_err(fault);
        if got.is_ok() {
            if let Some(mut cache) = self.0.try_lock() {
                cache.keep(id, held);
            }
        }
        got
    }
}
