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

//! A heap above a gigabyte is mapped in gigabyte pieces that meet end to end,
//! and one that does not meet, or does not map, leaves nothing mapped.

use super::span::map;
use crate::mem::{calls, reset};

const GIB: usize = 1 << 30;
const BASE: usize = 0x8000_0000;
const ENOMEM: isize = -12;
/// Read and write, private and anonymous, no file: every piece's arguments.
const PIECE_ARGS: (i32, i32, i32, i64) = (0x3, 0x22, -1, 0);

#[test]
fn the_provers_heap_is_three_pieces_that_meet() {
    reset(BASE, None, None);
    assert_eq!(map(2816 << 20).map(|p| p as usize), Some(BASE));
    let (maps, unmapped) = calls();
    let lens: Vec<usize> = maps.iter().map(|m| m.1).collect();
    assert_eq!((lens, unmapped), (vec![GIB, GIB, 768 << 20], Vec::new()));
    assert!(maps
        .iter()
        .all(|&(addr, _, p, f, fd, off)| addr == 0 && (p, f, fd, off) == PIECE_ARGS));
}

#[test]
fn a_piece_that_does_not_meet_the_last_undoes_them_all() {
    reset(BASE, None, Some(3));
    assert!(map(2816 << 20).is_none());
    let late = BASE + 2 * GIB + 4096;
    assert_eq!(calls().1, vec![(late, 768 << 20), (BASE, GIB), (BASE + GIB, GIB)]);
}

#[test]
fn a_piece_that_does_not_map_undoes_those_before_it() {
    reset(BASE, Some((2, ENOMEM)), None);
    assert!(map(2816 << 20).is_none());
    assert_eq!(calls().1, vec![(BASE, GIB)]);
    reset(BASE, Some((3, 0)), None);
    assert!(map(3 * GIB).is_none(), "a null third piece");
    assert_eq!(calls().1, vec![(BASE, GIB), (BASE + GIB, GIB)]);
    reset(BASE, Some((1, ENOMEM)), None);
    assert!(map(2816 << 20).is_none());
    assert_eq!(calls(), (vec![(0, GIB, 0x3, 0x22, -1, 0)], Vec::new()));
}
