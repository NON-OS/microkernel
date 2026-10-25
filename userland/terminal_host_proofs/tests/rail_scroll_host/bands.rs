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

//! Cutting and dropping bands against the rail.

use crate::paint::rail_band::{clip, visible, Band};
use crate::paint::rail_row::inside;
use crate::support::RAIL;

#[test]
fn a_band_is_cut_at_the_top_and_dropped_once_it_is_wholly_above() {
    assert_eq!(clip(10, 20), Some((10, 20)));
    assert_eq!(clip(0, 20), Some((0, 20)));
    assert_eq!(clip(-5, 20), Some((0, 15)));
    assert_eq!(clip(-20, 20), None);
    assert_eq!(clip(-21, 20), None);
    assert_eq!(clip(-1, 1), None);
}

#[test]
fn visibility_covers_exactly_the_bands_that_touch_the_rail() {
    let b = |y| Band { x: 0, y, w: 10, h: 20 };
    assert!(!visible(&b(-20), 100));
    assert!(visible(&b(-19), 100));
    assert!(visible(&b(99), 100));
    assert!(!visible(&b(100), 100));
    assert!(inside(RAIL, RAIL.x, RAIL.y));
    assert!(!inside(RAIL, RAIL.x, RAIL.y - 1));
}
