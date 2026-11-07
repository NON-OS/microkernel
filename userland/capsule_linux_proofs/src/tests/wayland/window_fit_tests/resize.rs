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

//! A window resized from its edges (wayland/fit/resize.rs): the rect the
//! guest is told, the edges a client may name, and when a resize is taken.

use crate::fit::place::origin;
use crate::fit::resize::MIN_CONTENT;
use crate::fit::{sized, Fit, Held, Mode, Resize, Sides};

/// A 400 by 300 window at (100, 80) with a frame 24 wide and 40 high.
const FRAME: (u32, u32) = (24, 40);
const BASE: (u32, u32, u32, u32) = (100, 80, 424, 340);
const SCREEN: (u32, u32) = (1920, 1080);
const TOP: u32 = 30;

fn held(sides: Sides) -> Resize {
    Resize { sides, press: (200, 150), base: BASE, limits: ((0, 0), (0, 0)) }
}

fn at(dx: i32, dy: i32) -> (i32, i32) {
    (200 + dx, 150 + dy)
}

#[test]
fn the_edges_held_follow_the_pointer_and_the_others_stay() {
    let br = Sides { right: true, bottom: true, ..Sides::default() };
    assert_eq!(sized(&held(br), at(50, 30), FRAME, TOP, SCREEN), (100, 80, 450, 330));
    let left = Sides { left: true, ..Sides::default() };
    assert_eq!(sized(&held(left), at(-30, 99), FRAME, TOP, SCREEN), (70, 80, 430, 300));
    let top = Sides { top: true, ..Sides::default() };
    assert_eq!(sized(&held(top), at(99, 20), FRAME, TOP, SCREEN), (100, 100, 400, 280));
}

#[test]
fn a_resize_keeps_the_least_size_the_screen_and_the_menubar() {
    let br = Sides { right: true, bottom: true, ..Sides::default() };
    let least = sized(&held(br), at(-5000, -5000), FRAME, TOP, SCREEN);
    assert_eq!(least, (100, 80, MIN_CONTENT.0, MIN_CONTENT.1));
    let most = sized(&held(br), at(9000, 9000), FRAME, TOP, SCREEN);
    assert_eq!(most, (100, 80, 1920 - 100 - 24, 1080 - 80 - 40), "to the screen's edge");
    let tl = Sides { left: true, top: true, ..Sides::default() };
    let up = sized(&held(tl), at(-5000, -5000), FRAME, TOP, SCREEN);
    assert_eq!(up, (0, TOP, 100 + 400, 80 + 300 - TOP), "the far edges stay");
    let down = sized(&held(tl), at(5000, 5000), FRAME, TOP, SCREEN);
    assert_eq!(down, (100 + 400 - MIN_CONTENT.0, 80 + 300 - MIN_CONTENT.1, 64, 48));
    let extreme = Resize {
        sides: br,
        press: (i32::MIN, i32::MIN),
        base: (u32::MAX, u32::MAX, 0, 0),
        limits: ((u32::MAX, u32::MAX), (1, 1)),
    };
    let _ = sized(&extreme, (i32::MAX, i32::MAX), (u32::MAX, u32::MAX), u32::MAX, (0, 0));
}

#[test]
fn a_client_names_an_edge_or_a_corner_and_nothing_else() {
    let s = |e| Sides::of_edge(e);
    assert_eq!(s(10), Sides { right: true, bottom: true, ..Sides::default() });
    assert_eq!(s(5), Sides { left: true, top: true, ..Sides::default() });
    assert_eq!(s(1), Sides { top: true, ..Sides::default() });
    for none in [0, 3, 12, 15, 16, 26, u32::MAX] {
        assert!(!s(none).any(), "{none}");
    }
}

#[test]
fn a_resize_is_told_only_to_a_window_in_its_own_size() {
    let mut fit = Fit::default();
    let rect = (100, 80, 450, 330);
    let told = fit.resize(rect, Held::default(), 7).expect("in its own size");
    assert_eq!((told.mode, told.rect, told.serial), (Mode::Normal, Some(rect), 7));
    fit.ack(7);
    assert_eq!(origin(fit.due(), Some((100, 80)), 474, 370), (100, 80));
    fit.shown();
    let fill = (0, 30, 1920, 1050);
    let s = fit.ask(Mode::Maximized, Some(rect), fill, 8).expect("maximize").serial;
    assert!(fit.resize(rect, Held::default(), 9).is_none(), "on its way to filling the display");
    fit.ack(s);
    fit.shown();
    assert!(fit.resize(rect, Held::default(), 10).is_none(), "maximized");
}

#[test]
fn a_window_off_the_screen_resized_from_its_low_edges_ends_at_the_screen() {
    let off = Resize {
        sides: Sides { left: true, top: true, ..Sides::default() },
        press: (1500, 700),
        base: (1500, 700, 1000, 800),
        limits: ((0, 0), (0, 0)),
    };
    let r = sized(&off, (0, 0), FRAME, TOP, SCREEN);
    assert_eq!(r, (0, TOP, 1920 - FRAME.0, 1080 - TOP - FRAME.1));
}

/// The client's own least and most size bound a resize: a left edge
/// dragged past the least stops there with the right edge kept, and a
/// corner dragged out stops at the most.
#[test]
fn a_resize_keeps_to_the_least_and_most_the_client_set() {
    let limits = ((300, 250), (500, 320));
    let left = Resize { limits, ..held(Sides { left: true, ..Sides::default() }) };
    let r = sized(&left, at(300, 0), FRAME, TOP, SCREEN);
    assert_eq!(r, (100 + 400 - 300, 80, 300, 300), "the right edge stays at the least");
    let br = Resize { limits, ..held(Sides { right: true, bottom: true, ..Sides::default() }) };
    assert_eq!(sized(&br, at(400, 400), FRAME, TOP, SCREEN), (100, 80, 500, 320));
    // A least over the most: the least.
    let odd = Resize { limits: ((600, 0), (500, 0)), ..br };
    assert_eq!(sized(&odd, at(0, 0), FRAME, TOP, SCREEN).2, 600);
}

/// A left edge dragged 50 to the left: the guest is told 450 wide at x 50,
/// its right edge where it was, at 524.
fn left_told(fit: &mut Fit, serial: u32) -> Held {
    let r = held(Sides { left: true, ..Sides::default() });
    let rect = sized(&r, at(-50, 0), FRAME, TOP, SCREEN);
    assert_eq!(rect, (50, 80, 450, 300));
    let h = Held::of(&r, rect, FRAME, TOP);
    assert_eq!(h, Held { right: Some(524), bottom: None, top: TOP });
    fit.resize(rect, h, serial).expect("in its own size");
    h
}

#[test]
fn a_client_that_draws_another_width_keeps_the_right_edge_a_left_resize_held() {
    let mut fit = Fit::default();
    left_told(&mut fit, 7);
    fit.ack(7);
    // It keeps whole cells, 440 wide, frame and all 464: placed from the
    // right edge at 524, not at the told x of 50.
    assert_eq!(fit.corner(Some((100, 80)), 464, 340), (60, 80));
    // Drawn as told: at the told corner.
    assert_eq!(fit.corner(Some((100, 80)), 474, 340), (50, 80));
    fit.shown();
    assert_eq!(fit.corner(Some((60, 80)), 464, 340), (60, 80), "nothing due: where it is");
}

#[test]
fn a_top_resize_keeps_the_bottom_and_never_goes_above_the_menubar() {
    let mut fit = Fit::default();
    let r = held(Sides { top: true, ..Sides::default() });
    let rect = sized(&r, at(0, -20), FRAME, TOP, SCREEN);
    assert_eq!(rect, (100, 60, 400, 320));
    let h = Held::of(&r, rect, FRAME, TOP);
    assert_eq!(h.bottom, Some(420));
    fit.resize(rect, h, 5).expect("in its own size");
    fit.ack(5);
    assert_eq!(fit.corner(None, 424, 350), (100, 70), "a shorter one: its bottom kept");
    assert_eq!(fit.corner(None, 424, 1000), (100, TOP), "a taller one: below the menubar");
}

#[test]
fn a_right_or_bottom_resize_holds_nothing_and_goes_to_the_told_corner() {
    let r = held(Sides { right: true, bottom: true, ..Sides::default() });
    let rect = sized(&r, at(50, 30), FRAME, TOP, SCREEN);
    let h = Held::of(&r, rect, FRAME, TOP);
    assert_eq!((h.right, h.bottom), (None, None));
    let mut fit = Fit::default();
    fit.resize(rect, h, 3).expect("in its own size");
    fit.ack(3);
    assert_eq!(fit.corner(None, 300, 200), (100, 80));
}

#[test]
fn what_a_resize_held_goes_with_its_configure_and_not_with_the_next() {
    let mut fit = Fit::default();
    left_told(&mut fit, 7);
    // Maximised before the guest acked the resize: the maximise is due, and
    // its corner is its own.
    let fill = (0, 30, 1920, 1050);
    let s = fit.ask(Mode::Maximized, Some((50, 80, 450, 300)), fill, 8).expect("max").serial;
    fit.ack(7);
    assert_eq!(fit.due(), None, "the resize was replaced");
    fit.ack(s);
    assert_eq!(fit.corner(Some((100, 80)), 1920, 1050), (0, 30));
    // Told again under a new serial, a resize keeps what it held.
    let mut fit = Fit::default();
    left_told(&mut fit, 7);
    let _ = fit.again(9);
    fit.ack(9);
    assert_eq!(fit.corner(Some((100, 80)), 464, 340), (60, 80));
}

#[test]
fn held_edges_saturate_rather_than_wrap() {
    let h = Held { right: Some(10), bottom: Some(5), top: 0 };
    assert_eq!(h.corner((3, 3), (u32::MAX, u32::MAX)), (0, 0));
    let r = held(Sides { left: true, top: true, ..Sides::default() });
    let far = Held::of(&r, (u32::MAX, u32::MAX, 9, 9), FRAME, TOP);
    assert_eq!((far.right, far.bottom), (Some(u32::MAX), Some(u32::MAX)));
}
