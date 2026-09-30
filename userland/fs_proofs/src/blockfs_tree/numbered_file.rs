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

//! A file of numbered data blocks on the counting disk, closed as the
//! kernel closes one: pointer blocks, then the index block last.

use super::disk::Disk;
use super::file_consts::DATA_BYTES;
use super::index_block::encode;
use super::mem::byte_at;
use super::tree_store::BlockStore;
use super::tree_writer::TreeWriter;

/// Where a file's node and index block lie, and its size.
#[derive(Clone, Copy)]
pub struct File {
    pub node: u64,
    pub index: u64,
    pub size: u64,
}

/// Write `size` bytes as never-written data blocks, then the pointer blocks
/// and the index block last. The file, and each data block's LBA in order.
pub fn write_numbered(disk: &mut Disk, node: u64, size: u64) -> (File, Vec<u64>) {
    let mut tree = TreeWriter::new();
    let mut data = Vec::new();
    for _ in 0..size.div_ceil(DATA_BYTES as u64) {
        let lba = disk.alloc().unwrap();
        tree.push(disk, lba).unwrap();
        data.push(lba);
    }
    tree.finish(disk).unwrap();
    let index = disk.alloc().unwrap();
    disk.put(index, &encode(&tree.root, tree.data_blocks)).unwrap();
    (File { node, index, size }, data)
}

/// The byte at `at` of a file whose data blocks are `data`, as written.
pub fn expect(data: &[u64], at: u64) -> u8 {
    let (n, k) = (at / DATA_BYTES as u64, at % DATA_BYTES as u64);
    byte_at(data[n as usize] * DATA_BYTES as u64 + k)
}
