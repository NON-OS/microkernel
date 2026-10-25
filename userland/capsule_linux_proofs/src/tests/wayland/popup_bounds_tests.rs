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

//! C8v: a popup is inside its window. Larger than the window, it is cut to
//! it, or without that adjustment clipped where it is drawn; never drawn
//! past the window's frame. And the pointer reaches a popup only inside
//! it, and only one holding its grab.

use crate::wayland_popup::place::place;
use crate::wayland_popup::positioner::{set, Positioner, RESIZE_X, RESIZE_Y};
use crate::wayland_popup::stack::{Popup, Popups};
use crate::wayland_sub::blend::{lay, Plane};

#[test]
fn a_popup_larger_than_its_window_is_cut_to_it_or_clipped_where_drawn() {
    let window = (0, 0, 60, 40);
    let mut p = Positioner::default();
    assert!(set(&mut p, 1, &[100, 90]) && set(&mut p, 2, &[10, 10, 1, 1]) && set(&mut p, 4, &[8]));
    assert_eq!(place(&p, window), (10, 10, 100, 90), "no adjustment: as asked");
    assert!(set(&mut p, 5, &[RESIZE_X | RESIZE_Y]));
    assert_eq!(place(&p, window), (10, 10, 50, 30), "cut to the window");
    // Drawn as asked anyway, it stays inside the frame: nothing past it.
    let frame_plane = Plane { w: 60, h: 40, stride: 60 * 4 };
    let mut frame = vec![0u8; (frame_plane.stride * frame_plane.h) as usize + 64];
    let layer = vec![0xFF; 100 * 4 * 90];
    let (head, tail) = frame.split_at_mut(frame_plane.stride as usize * 40);
    lay(head, frame_plane, &layer, Plane { w: 100, h: 90, stride: 400 }, (10, 10), true);
    assert!(tail.iter().all(|b| *b == 0), "past the frame");
    assert!(head
        .chunks(4)
        .enumerate()
        .all(|(i, px)| (i % 60 >= 10 && i / 60 >= 10) == (px[3] == 0xFF)));
}

#[test]
fn the_pointer_reaches_only_a_grabbing_popup_and_only_inside_it() {
    let at = |id, rect, grabbed| Popup { id, surface: id + 100, parent: 1, rect, grabbed };
    let p = Popups {
        list: vec![
            at(5, (10, 10, 20, 20), true),
            at(6, (20, 20, 20, 20), true),
            at(7, (0, 0, 60, 40), false),
        ],
        positioners: Vec::new(),
        last_press: None,
    };
    assert_eq!(p.under(25, 25), Some((106, (5, 5))), "the topmost of two");
    assert_eq!(p.under(12, 12), Some((105, (2, 2))));
    assert_eq!(p.under(30, 10), None, "past 5's right edge: the toplevel's");
    assert_eq!(p.under(2, 2), None, "7 holds no grab");
    assert_eq!(p.under(i32::MIN, i32::MAX), None);
}
