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

//! What a write-back takes: whole pages of the map, never past the file's
//! end, and of those only the pages whose bytes differ from the file's.

use super::super::random::Regs;
use super::support::{map, P};
use crate::linux::file::shared_maps::spans::{back_range, differing};

#[test]
fn a_write_back_is_whole_pages_cut_at_the_file_end() {
    let e = map(1, 16, 3, "/f", 0);
    assert_eq!(back_range(&e, (100, 200), 3 * P), Some((0, P)), "the page a read touches");
    assert_eq!(back_range(&e, (0, 3 * P), 5000), Some((0, 5000)), "never past the end");
    assert_eq!(back_range(&e, (2 * P, 3 * P), 5000), None, "a page wholly past the end");
    assert_eq!(back_range(&e, (0, 3 * P), 0), None, "an emptied file takes nothing");
    assert_eq!(back_range(&e, (4 * P, 5 * P), 9 * P), None, "a read past the map");
}

#[test]
fn only_the_pages_that_differ_are_taken() {
    let file = vec![7u8; 3 * P as usize + 100];
    let mut map = file.clone();
    assert!(differing(&map, &file).is_empty(), "nothing changed, nothing written");
    map[P as usize + 9] = 1;
    assert_eq!(differing(&map, &file), vec![(P as usize, 2 * P as usize)]);
    map[3 * P as usize + 99] = 1;
    let last = (3 * P as usize, 3 * P as usize + 100);
    assert_eq!(differing(&map, &file), vec![(P as usize, 2 * P as usize), last]);
}

#[test]
fn past_the_end_of_the_file_nothing_is_compared() {
    let file = vec![0u8; 100];
    let mut map = vec![0u8; P as usize];
    map[200] = 9;
    assert!(differing(&map, &file).is_empty(), "a byte past the file's end");
    map[99] = 9;
    assert_eq!(differing(&map, &file), vec![(0, 100)]);
}

/// Over random bytes, every page taken differs, every page left is equal,
/// and the pages taken never pass the shorter of the two.
#[test]
fn every_page_taken_differs_and_every_page_left_is_equal() {
    let mut r = Regs::new(0x5AED_0DD5);
    for _ in 0..300 {
        let n = r.small(5 * P) as usize;
        let file: Vec<u8> = (0..n).map(|_| r.small(2) as u8).collect();
        let map: Vec<u8> = (0..r.small(6 * P) as usize).map(|_| r.small(2) as u8).collect();
        let taken = differing(&map, &file);
        let m = n.min(map.len());
        for lo in (0..m).step_by(P as usize) {
            let hi = (lo + P as usize).min(m);
            assert_eq!(taken.contains(&(lo, hi)), map[lo..hi] != file[lo..hi], "page {lo}");
        }
        assert!(taken.iter().all(|&(lo, hi)| lo % P as usize == 0 && hi <= m));
    }
}
