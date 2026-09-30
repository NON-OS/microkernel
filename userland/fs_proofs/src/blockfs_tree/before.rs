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

//! A file read as every read was made before the read cache: a fresh run
//! and pointer path for each piece, and pointer blocks fetched through the
//! run, throwing away the data blocks it had fetched ahead.

use super::disk::Disk;
use super::index_block::{decode, Index};
use super::numbered_file::File;
use super::run::{Run, Source};
use super::tree_range::read_range;
use super::tree_reader::TreeReader;
use super::tree_store::{Block, BlockSource};

/// Pointer blocks through the run, as every read did before.
struct Before<'a>(Source<'a>);
impl BlockSource for Before<'_> {
    type Error = &'static str;
    fn get(&mut self, lba: u64) -> Result<Block, &'static str> {
        self.0.get(lba)
    }
}

/// Read `f` from `at` into `out` as one read was made before.
pub fn read_before(disk: &mut Disk, f: File, at: u64, out: &mut [u8]) -> usize {
    let root = match decode(&disk.get(f.index).unwrap()) {
        Some(Index::Tree(root)) => root,
        _ => panic!("not a tree file's index block"),
    };
    let mut run = Run::default();
    let mut source = Before(Source { disk, run: &mut run });
    read_range(&mut source, &mut TreeReader::new(root), f.size, at, out).unwrap()
}
