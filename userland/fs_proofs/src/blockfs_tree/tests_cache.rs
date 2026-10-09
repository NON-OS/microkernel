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

//! A model read in 1 MiB pieces, as a guest loads one: the right bytes, and
//! each sealed sector fetched about once where it used to be twice or more.

use super::before::read_before;
use super::cached::read_cached;
use super::disk::Disk;
use super::file_cache::FileCache;
use super::numbered_file::{expect, write_numbered};
use super::run::RUN;

const PIECE: usize = 1 << 20;

#[test]
fn a_model_read_in_mib_pieces_fetches_each_sector_about_once() {
    let mut disk = Disk::default();
    let (f, data) = write_numbered(&mut disk, 1, 491_400_032);
    let (sectors, pointers) = (disk.last, disk.last - 1 - data.len() as u64);
    let (mut cache, mut buf) = (FileCache::new(), vec![0u8; PIECE]);
    (disk.fetched, disk.requests) = (0, 0);
    let mut at = 0u64;
    while at < f.size {
        let n = read_cached(&mut disk, &mut cache, f, at, &mut buf);
        assert_eq!(n as u64, (PIECE as u64).min(f.size - at));
        for (k, b) in buf[..n].iter().enumerate() {
            assert_eq!(*b, expect(&data, at + k as u64), "byte {}", at + k as u64);
        }
        at += n as u64;
    }
    /*
     * Every sector once through the runs, the pointer blocks once more on
     * their own, and the index block: under 2% over the file's sectors.
     */
    assert!(disk.fetched <= sectors + pointers + RUN + 1, "{} of {sectors}", disk.fetched);
    assert!(disk.fetched * 100 <= sectors * 102, "{} of {sectors}", disk.fetched);
    assert!(disk.requests <= sectors / RUN + pointers + 2, "{} requests", disk.requests);
    /*
     * The same pieces as each was read before: a fresh run and path per
     * piece, and pointer blocks through the run.
     */
    (disk.fetched, at) = (0, 0);
    while at < f.size {
        at += read_before(&mut disk, f, at, &mut buf) as u64;
    }
    assert!(disk.fetched > 2 * sectors, "{} of {sectors} before", disk.fetched);
}
