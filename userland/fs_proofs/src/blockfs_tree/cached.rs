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

//! A file on the counting disk read as the kernel's ReadCache reads it:
//! the state kept under the file's id, taken out for a read and put back.

use super::disk::Disk;
use super::file_cache::{FileCache, FileId, Held};
use super::index_block::{decode, Index};
use super::numbered_file::File;
use super::run::{Run, Source};
use super::tree_range::read_range;
use super::tree_reader::TreeReader;
use super::tree_store::BlockSource;

/// Read `f` from `offset` into `out` through `cache`, as ReadCache::read_at.
pub fn read_cached(
    disk: &mut Disk,
    cache: &mut FileCache<Run>,
    f: File,
    offset: u64,
    out: &mut [u8],
) -> usize {
    let id = id(disk, f);
    let mut held = match cache.take(&id) {
        Some(held) => held,
        None => match decode(&disk.get(f.index).unwrap()) {
            Some(Index::Tree(root)) => {
                Held { reader: TreeReader::new(root), ahead: Run::default() }
            }
            _ => panic!("not a tree file's index block"),
        },
    };
    let mut source = Source { disk, run: &mut held.ahead };
    let n = read_range(&mut source, &mut held.reader, f.size, offset, out).unwrap();
    cache.keep(id, held);
    n
}

/// The id the kernel reads `f` under on `disk` as it is now.
pub fn id(disk: &Disk, f: File) -> FileId {
    let (node_lba, index_lba, size) = (f.node, f.index, f.size);
    FileId { volume: [7; 16], generation: 1, epoch: disk.epoch, node_lba, index_lba, size }
}
