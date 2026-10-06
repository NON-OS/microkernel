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

//! The read cache's id: every part of it names what was read, a state is
//! handed back only for the very id it was kept under, and a file written
//! again is read from its new blocks.

use super::cached::read_cached;
use super::disk::Disk;
use super::file_cache::{FileCache, FileId, Held};
use super::file_consts::{DATA_BYTES, ROOTS};
use super::numbered_file::{expect, write_numbered};
use super::run::Run;
use super::tree_reader::TreeReader;

#[test]
fn a_kept_state_is_given_back_only_for_the_same_id() {
    let base =
        FileId { volume: [1; 16], generation: 2, epoch: 3, node_lba: 4, index_lba: 5, size: 6 };
    let others = [
        FileId { volume: [9; 16], ..base },
        FileId { generation: 9, ..base },
        FileId { epoch: 9, ..base },
        FileId { node_lba: 9, ..base },
        FileId { index_lba: 9, ..base },
        FileId { size: 9, ..base },
    ];
    let held = || Held { reader: TreeReader::new([0; ROOTS]), ahead: Run::default() };
    let mut cache = FileCache::new();
    for other in others {
        cache.keep(base, held());
        assert!(cache.take(&other).is_none());
        assert!(cache.take(&base).is_none(), "a miss drops what was kept");
    }
    cache.keep(base, held());
    assert!(cache.take(&base).is_some());
    assert!(cache.take(&base).is_none(), "taken out while in use");
}

#[test]
fn a_file_written_again_is_read_from_its_new_blocks() {
    let mut disk = Disk::default();
    let (f, _) = write_numbered(&mut disk, 1, 5000 * DATA_BYTES as u64 + 9);
    let (mut cache, mut out) = (FileCache::new(), vec![0u8; 3 * DATA_BYTES]);
    read_cached(&mut disk, &mut cache, f, 4000 * DATA_BYTES as u64, &mut out);
    let (g, data) = write_numbered(&mut disk, 1, 4200 * DATA_BYTES as u64);
    let at = 4000 * DATA_BYTES as u64 + 11;
    let n = read_cached(&mut disk, &mut cache, g, at, &mut out);
    for (k, b) in out[..n].iter().enumerate() {
        assert_eq!(*b, expect(&data, at + k as u64));
    }
}
