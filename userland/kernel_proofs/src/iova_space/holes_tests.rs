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

use super::holes::{give_back, take_hole, Holes};
use super::space::IOVA_BASE;

const PAGE: u64 = 4096;

#[test]
fn a_run_below_the_top_becomes_a_hole_a_same_size_map_reuses() {
    let (mut next, mut holes) = (IOVA_BASE + 3 * PAGE, Holes::new());
    give_back(&mut next, &mut holes, IOVA_BASE + PAGE, PAGE);
    assert_eq!(next, IOVA_BASE + 3 * PAGE);
    assert_eq!(holes, [(IOVA_BASE + PAGE, IOVA_BASE + 2 * PAGE)]);
    assert_eq!(take_hole(&mut holes, PAGE), Some(IOVA_BASE + PAGE));
    assert!(holes.is_empty());
}

#[test]
fn the_top_run_lowers_the_pointer_through_the_holes_it_meets() {
    let (mut next, mut holes) = (IOVA_BASE + 4 * PAGE, Holes::new());
    give_back(&mut next, &mut holes, IOVA_BASE + 2 * PAGE, PAGE);
    give_back(&mut next, &mut holes, IOVA_BASE, PAGE);
    assert_eq!(holes.len(), 2);
    give_back(&mut next, &mut holes, IOVA_BASE + 3 * PAGE, PAGE);
    // Down past the hole at page 2; page 1 is live, so it stops there.
    assert_eq!(next, IOVA_BASE + 2 * PAGE);
    assert_eq!(holes, [(IOVA_BASE, IOVA_BASE + PAGE)]);
    give_back(&mut next, &mut holes, IOVA_BASE + PAGE, PAGE);
    assert_eq!((next, holes.len()), (IOVA_BASE, 0));
}

#[test]
fn neighbouring_holes_merge_on_both_sides() {
    let (mut next, mut holes) = (IOVA_BASE + 8 * PAGE, Holes::new());
    give_back(&mut next, &mut holes, IOVA_BASE + 2 * PAGE, PAGE);
    give_back(&mut next, &mut holes, IOVA_BASE + 4 * PAGE, PAGE);
    give_back(&mut next, &mut holes, IOVA_BASE + 3 * PAGE, PAGE);
    assert_eq!(holes, [(IOVA_BASE + 2 * PAGE, IOVA_BASE + 5 * PAGE)]);
}

#[test]
fn the_first_hole_that_fits_is_carved_from_its_low_end() {
    let mut holes: Holes = vec![(PAGE, 2 * PAGE), (4 * PAGE, 8 * PAGE)];
    assert_eq!(take_hole(&mut holes, 2 * PAGE), Some(4 * PAGE));
    assert_eq!(holes, [(PAGE, 2 * PAGE), (6 * PAGE, 8 * PAGE)]);
    assert_eq!(take_hole(&mut holes, 3 * PAGE), None);
}

#[test]
fn nothing_is_taken_or_given_for_nothing() {
    let (mut next, mut holes): (u64, Holes) = (IOVA_BASE + PAGE, vec![(0, PAGE)]);
    assert_eq!(take_hole(&mut holes, 0), None);
    give_back(&mut next, &mut holes, IOVA_BASE, 0);
    give_back(&mut next, &mut holes, u64::MAX, 2);
    assert_eq!((next, holes.len()), (IOVA_BASE + PAGE, 1));
}
