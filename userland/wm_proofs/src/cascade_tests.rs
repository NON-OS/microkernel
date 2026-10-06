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

use super::cascade::cascade;
use super::constants::{dock_for, menubar_for};
use crate::geometry::Rect;

const SCREENS: [(u32, u32); 5] =
    [(1280, 800), (1920, 1080), (1280, 1024), (2560, 1440), (3840, 2160)];

fn size(width: u32, height: u32) -> Rect {
    Rect { x: 0, y: 0, width, height }
}

/* A window that fits the work area opens inside it, at every scale and for
 * every slot of the cascade: never under the menubar, never over the dock. */
#[test]
fn a_window_that_fits_stays_between_the_bar_and_the_dock() {
    for (w, h) in SCREENS {
        let bar = menubar_for(w, h);
        let dock = dock_for(w, h);
        let req = size(w / 2, (h - bar - dock) / 2);
        for open in 0..12 {
            let r = cascade(w, h, open, req);
            assert!(r.y >= bar, "{w}x{h} slot {open}: y {} under the bar {bar}", r.y);
            assert!(r.y + r.height <= h - dock, "{w}x{h} slot {open}: over the dock");
            assert!(r.x + r.width <= w, "{w}x{h} slot {open}: past the right edge");
        }
    }
}

/* The first window of a run is centred in the work area. */
#[test]
fn the_first_window_is_centred_in_the_work_area() {
    let (w, h) = (1920, 1080);
    let bar = menubar_for(w, h);
    let dock = dock_for(w, h);
    let r = cascade(w, h, 0, size(800, 600));
    assert_eq!(r.x, (w - 800) / 2);
    assert_eq!(r.y, bar + (h - bar - dock - 600) / 2);
}

/* Later windows step down and right, and the run starts over after five. */
#[test]
fn the_cascade_steps_and_wraps() {
    let first = cascade(1920, 1080, 0, size(640, 400));
    let second = cascade(1920, 1080, 1, size(640, 400));
    let sixth = cascade(1920, 1080, 5, size(640, 400));
    assert_eq!(second.x - first.x, 64);
    assert_eq!(second.y - first.y, 64);
    assert_eq!((sixth.x, sixth.y), (first.x, first.y));
}

/* A window taller than the work area keeps its title bar below the menubar
 * and only then reaches down over the dock's band; one taller than the screen
 * below the bar still keeps it, and runs off the foot instead. */
#[test]
fn a_tall_window_keeps_its_title_bar_reachable() {
    let (w, h) = (1280, 800);
    let bar = menubar_for(w, h);
    let r = cascade(w, h, 3, size(600, 700));
    assert!(r.y >= bar);
    assert!(r.y + r.height <= h);
    for open in 0..5 {
        assert_eq!(cascade(w, h, open, size(600, 790)).y, bar);
    }
}
