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

//! Reads after a write: nothing a read kept is used to answer them.

use super::cached::{id, read_cached};
use super::disk::Disk;
use super::file_cache::FileCache;
use super::file_consts::{DATA_BYTES, DIRECT_SLOTS};
use super::mem::numbered;
use super::numbered_file::{expect, write_numbered};
use super::run::Source;
use super::tree_ptrs::encode;
use super::tree_range::read_range;
use super::tree_store::{BlockSource, BlockStore};

#[test]
fn a_block_written_inside_a_kept_run_is_read_as_written() {
    let mut disk = Disk::default();
    let (f, data) = write_numbered(&mut disk, 1, 300 * DATA_BYTES as u64);
    let (mut cache, mut out) = (FileCache::new(), vec![0u8; DATA_BYTES]);
    read_cached(&mut disk, &mut cache, f, 0, &mut out);
    let before = id(&disk, f);
    let fresh = [0xa5u8; DATA_BYTES];
    disk.put(data[5], &fresh).unwrap();
    /*
     * The run kept before the write still holds the old block: read under
     * the old id, it would answer with the old bytes.
     */
    let at = 5 * DATA_BYTES as u64;
    let mut held = cache.take(&before).expect("kept under the id before the write");
    let mut source = Source { disk: &mut disk, run: &mut held.ahead };
    read_range(&mut source, &mut held.reader, f.size, at, &mut out).unwrap();
    assert_eq!(out[..], numbered(data[5])[..]);
    cache.keep(before, held);
    /*
     * The write moved the id, so the next read starts afresh.
     */
    read_cached(&mut disk, &mut cache, f, at, &mut out);
    assert_eq!(out[..], fresh[..]);
}

#[test]
fn a_pointer_block_written_over_is_followed_where_it_now_points() {
    let mut disk = Disk::default();
    let (f, data) = write_numbered(&mut disk, 1, 200 * DATA_BYTES as u64);
    let (mut cache, mut out) = (FileCache::new(), vec![0u8; DATA_BYTES]);
    let at = (DIRECT_SLOTS as u64 + 3) * DATA_BYTES as u64;
    read_cached(&mut disk, &mut cache, f, at, &mut out);
    /*
     * The single-level tree's block, which the kept reader holds, now
     * points every entry at the file's first data block.
     */
    let root = match super::index_block::decode(&disk.get(f.index).unwrap()) {
        Some(super::index_block::Index::Tree(root)) => root,
        _ => panic!("not a tree file's index block"),
    };
    disk.put(root[DIRECT_SLOTS], &encode(&[data[0]; 60])).unwrap();
    read_cached(&mut disk, &mut cache, f, at, &mut out);
    for (k, b) in out.iter().enumerate() {
        assert_eq!(*b, expect(&data, k as u64));
    }
}
