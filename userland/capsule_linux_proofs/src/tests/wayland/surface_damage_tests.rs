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

//! A commit shows only what its client said changed: the box it named, all
//! of a buffer attached with nothing named, and nothing for a bare commit.

use crate::damage::{at_commit, Damage};
use crate::damage_lay::lay;

#[test]
fn boxes_said_are_one_box_round_them() {
    let d = Damage::None.add((10, 20, 5, 5), 1).add((40, 2, 10, 3), 1);
    assert_eq!(d, Damage::Rect(10, 2, 50, 25));
}

#[test]
fn surface_damage_is_in_buffer_pixels_at_its_scale() {
    assert_eq!(Damage::None.add((3, 4, 1, 1), 2), Damage::Rect(6, 8, 8, 10));
}

#[test]
fn everything_named_with_max_sizes_is_the_whole_buffer() {
    let d = Damage::None.add((0, 0, i32::MAX, i32::MAX), 1);
    assert_eq!(d.on(800, 600), Damage::Whole);
    assert_eq!(Damage::None.add((-5, -5, 20, 20), 1), Damage::Rect(0, 0, 15, 15));
}

#[test]
fn an_empty_or_outside_box_changes_nothing() {
    assert_eq!(Damage::None.add((1, 1, 0, 4), 1), Damage::None);
    assert_eq!(Damage::None.add((900, 10, 4, 4), 1).on(800, 600), Damage::None);
    assert_eq!(Damage::Whole.add((1, 1, 4, 4), 1), Damage::Whole);
}

#[test]
fn a_new_buffer_with_nothing_said_is_all_new() {
    assert_eq!(at_commit(Damage::None, true), Damage::Whole);
    assert_eq!(at_commit(Damage::None, false), Damage::None);
    assert_eq!(at_commit(Damage::Rect(0, 0, 1, 1), true), Damage::Rect(0, 0, 1, 1));
}

#[test]
fn only_the_box_is_laid_at_the_content_corner() {
    // A 4x3 client, 16 bytes a row, in a 6x5 window with the content at (1, 1).
    let client: Vec<u8> = (0..12u32).flat_map(|i| (i | 0xFF00_0000).to_le_bytes()).collect();
    let mut out = vec![0u8; 6 * 5 * 4];
    lay(&mut out, (24, 1, 1), &client, 16, (1, 1, 3, 2), |px| px);
    let at =
        |x: usize, y: usize| u32::from_le_bytes(out[(y * 6 + x) * 4..][..4].try_into().unwrap());
    assert_eq!((at(2, 2), at(3, 2)), (0xFF00_0005, 0xFF00_0006));
    assert_eq!((at(1, 2), at(4, 2), at(2, 1), at(2, 3)), (0, 0, 0, 0));
}

#[test]
fn rows_read_apart_are_put_back_in_their_box_only() {
    // Rows 1 and 2 of a 3x4 buffer, 12 bytes a row, read apart; the box is x 1..2.
    let rows = vec![7u8; 2 * 12];
    let mut kept = vec![1u8; 4 * 12];
    lay(&mut kept, (12, 0, 1), &rows, 12, (1, 0, 2, 2), |_| 0xAABB_CCDD);
    let px = |x: usize, y: usize| kept[y * 12 + x * 4];
    assert_eq!((px(1, 1), px(1, 2)), (0xDD, 0xDD));
    assert_eq!((px(0, 1), px(2, 1), px(1, 0), px(1, 3)), (1, 1, 1, 1));
}
