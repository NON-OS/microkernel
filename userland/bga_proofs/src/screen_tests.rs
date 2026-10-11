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

//! Who may present, and which rects a present may name.

use crate::dispi::Mode;
use crate::gfx::{Rect, Screen, RESOURCE_ID};
use crate::protocol::{E_BUSY, E_INVAL};

const M: Mode = Mode { width: 640, height: 480, stride: 640 * 4 };
const COMPOSITOR: u32 = 7;
const OTHER: u32 = 9;

fn r(x: u32, y: u32, w: u32, h: u32) -> Rect {
    Rect { x, y, w, h }
}

#[test]
fn the_first_to_ask_holds_the_screen_and_no_one_else_presents() {
    let mut s = Screen::new(M);
    assert_eq!(s.held(COMPOSITOR, RESOURCE_ID), Err(E_BUSY), "presents before taking");
    assert_eq!(s.take(COMPOSITOR), Ok(()));
    assert_eq!(s.take(COMPOSITOR), Ok(()), "asking again is the same holder");
    assert_eq!(s.take(OTHER), Err(E_BUSY));
    assert_eq!(s.held(OTHER, RESOURCE_ID), Err(E_BUSY));
    assert_eq!(s.held(COMPOSITOR, RESOURCE_ID), Ok(()));
    assert_eq!(s.held(COMPOSITOR, 2), Err(E_INVAL), "a resource this has not");
}

#[test]
fn a_rect_must_be_inside_the_mode_and_not_empty() {
    let s = Screen::new(M);
    assert_eq!(s.inside(r(0, 0, 640, 480)), Ok(()));
    assert_eq!(s.inside(r(639, 479, 1, 1)), Ok(()));
    for bad in
        [r(0, 0, 0, 1), r(0, 0, 1, 0), r(1, 0, 640, 1), r(0, 1, 1, 480), r(u32::MAX, 0, 2, 1)]
    {
        assert_eq!(s.inside(bad), Err(E_INVAL), "{bad:?}");
    }
}

#[test]
fn the_source_rows_must_end_inside_the_shadow_on_a_pixel() {
    let s = Screen::new(M);
    let last_row = 479 * M.stride as u64;
    assert_eq!(s.source(r(0, 0, 640, 480), 0), Ok(()));
    assert_eq!(s.source(r(0, 0, 640, 1), last_row), Ok(()));
    assert_eq!(s.source(r(0, 0, 640, 1), last_row + 4), Err(E_INVAL), "one pixel past");
    assert_eq!(s.source(r(0, 0, 1, 1), 2), Err(E_INVAL), "not a pixel boundary");
    assert_eq!(s.source(r(0, 0, 1, 2), u64::MAX - 8), Err(E_INVAL), "overflow");
}

#[test]
fn scanout_zero_shows_the_whole_mode_and_nothing_else() {
    let s = Screen::new(M);
    assert_eq!(s.whole(0, r(0, 0, 640, 480)), Ok(()));
    assert_eq!(s.whole(1, r(0, 0, 640, 480)), Err(E_INVAL), "one head");
    assert_eq!(s.whole(0, r(0, 0, 320, 240)), Err(E_INVAL), "the mode is fixed");
}
