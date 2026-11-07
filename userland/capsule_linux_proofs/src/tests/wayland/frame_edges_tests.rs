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

//! A press just inside a Linux window's border resizes it, as on a native
//! window: the frame's edge bands reach in over the content's outer pixels.
//! Live, presses 1 to 3 px inside Telegram's border reached the client and
//! only the shadow margin outside it resized.

use crate::wayland_frame::geometry::{around_at, Around, OVER_CONTENT};

const POINTER: &str =
    include_str!("../../../../capsule_linux/src/linux/wayland/present/frame/pointer.rs");
const Q: u32 = 4;
const C: (u32, u32) = (1051, 845);

fn window() -> Around {
    around_at(C.0, C.1, Q, false, false)
}

#[test]
fn just_inside_the_borders_are_edges_over_the_content() {
    let a = window();
    let (right, bottom) = ((a.cx + C.0) as i32 - 2, (a.cy + C.1) as i32 - 2);
    let mid_y = (a.cy + C.1 / 2) as i32;
    assert!(a.on_content(right, mid_y, C.0, C.1).is_some(), "on the content's last columns");
    let e = a.edges(right, mid_y, C, Q).expect("the right edge");
    assert!(e.right && !e.left && !e.top && !e.bottom);
    let corner = a.edges(right, bottom, C, Q).expect("the corner");
    assert!(corner.right && corner.bottom);
    assert!(a.edges(a.cx as i32 + 1, mid_y, C, Q).expect("the left edge").left);
    for d in 1..=OVER_CONTENT as i32 {
        assert!(a.edges((a.cx + C.0) as i32 - d, mid_y, C, Q).is_some(), "{d} px inside");
    }
}

#[test]
fn past_the_band_the_content_is_the_clients_scroll_bar_and_all() {
    let a = window();
    let mid_y = (a.cy + C.1 / 2) as i32;
    let past = OVER_CONTENT as i32 + 1;
    assert_eq!(a.edges((a.cx + C.0) as i32 - past, mid_y, C, Q), None, "a scroll bar's column");
    assert_eq!(a.edges(a.cx as i32 + past, mid_y, C, Q), None, "5 px in from the left");
    let (mx, my) = ((a.cx + C.0 / 2) as i32, (a.cy + C.1 / 2) as i32);
    assert_eq!(a.edges(mx, my, C, Q), None, "the middle");
    assert_eq!(a.edges(mx, (a.cy + C.1) as i32 - past, C, Q), None, "above the bottom band");
    assert_eq!(a.edges(mx, (a.cy / 2) as i32, C, Q), None, "the title bar moves the window");
}

#[test]
fn the_band_grows_with_the_scale_and_off_the_content_is_the_toolkits() {
    let a = around_at(C.0, C.1, 8, false, false);
    let mid_y = (a.cy + C.1 / 2) as i32;
    let d = 2 * OVER_CONTENT as i32;
    assert!(a.edges((a.cx + C.0) as i32 - d, mid_y, C, 8).is_some(), "8 px at double scale");
    assert_eq!(a.edges((a.cx + C.0) as i32 - d - 1, mid_y, C, 8), None);
    let w = window();
    assert!(w.edges(w.w as i32 - 1, mid_y, C, Q).expect("the shadow margin").right);
}

#[test]
fn a_bare_or_maximised_window_or_a_point_off_it_has_no_edge() {
    let a = window();
    let right = (a.cx + C.0) as i32 - 2;
    assert_eq!(Around::bare(C.0, C.1).edges(1049, 400, C, Q), None, "no frame's bands");
    let max = around_at(C.0, C.1, Q, false, true);
    assert_eq!(max.edges((max.cx + C.0) as i32 - 2, (max.cy + C.1 / 2) as i32, C, Q), None);
    assert_eq!(a.edges(-1, 400, C, Q), None);
    assert_eq!(a.edges(right, -1, C, Q), None);
}

#[test]
fn a_left_press_there_resizes_and_every_other_event_stays_the_clients() {
    let content = POINTER.split("if a.on_content(x, y, cw, ch).is_some() {").nth(1).expect("arm");
    let arm = content.split("\n    }").next().expect("its body");
    assert!(arm.contains("return down && code == LEFT && begin_edges(guest, a, (cw, ch), (x, y));"));
    let begin = POINTER.split("fn begin_edges(").nth(1).expect("begin_edges");
    assert!(begin.contains("edges(&guest.scene, a, (x, y), content)"));
    assert!(begin.contains("resize::begin(guest, sides, (x, y))"));
    let motion = POINTER.split("pub fn motion(").nth(1).expect("motion");
    assert!(
        motion.contains("let held = guest.scene.dnd.pressed;"),
        "a held drag keeps the pointer"
    );
    assert!(motion.contains("edges(&guest.scene, a, (x, y), (cw, ch)).filter(|_| !held && sizes)"));
    assert!(motion.contains("a.on_content(x, y, cw, ch).filter(|_| over.is_none())"));
    assert!(motion.contains("|e| e.cursor()"), "the cursor the press would take");
}

/// A window that draws its own frame is cut to its geometry, so its own
/// resize margin takes no press; the band just inside its edge resizes it.
#[test]
fn a_window_that_draws_its_own_frame_has_the_band_just_inside_its_edge() {
    let a = Around::bare(C.0, C.1);
    let (r, b, mid_y) = (C.0 as i32 - 1, C.1 as i32 - 1, C.1 as i32 / 2);
    let e = a.inner_edges(r - 1, mid_y, Q).expect("2 px inside the right edge");
    assert!(e.right && !e.left && !e.top && !e.bottom);
    let corner = a.inner_edges(r, b, Q).expect("the corner");
    assert!(corner.right && corner.bottom);
    assert!(a.inner_edges(0, mid_y, Q).expect("the left edge").left);
    assert!(a.inner_edges(C.0 as i32 / 2, 0, Q).expect("the top edge").top);
    for d in 1..=OVER_CONTENT as i32 {
        assert!(a.inner_edges(C.0 as i32 - d, mid_y, Q).is_some(), "{d} px inside");
    }
    let past = OVER_CONTENT as i32 + 1;
    assert_eq!(a.inner_edges(C.0 as i32 - past, mid_y, Q), None, "the client's past the band");
    assert_eq!(a.inner_edges(C.0 as i32 / 2, mid_y, Q), None, "the middle");
    assert!(a.inner_edges(C.0 as i32 - 2 * OVER_CONTENT as i32, mid_y, 8).is_some(), "scaled");
}

#[test]
fn no_inner_band_off_the_window_or_on_a_framed_one() {
    let a = Around::bare(C.0, C.1);
    assert_eq!(a.inner_edges(-1, 400, Q), None);
    assert_eq!(a.inner_edges(400, -1, Q), None);
    assert_eq!(a.inner_edges(C.0 as i32, 400, Q), None, "one past the right edge");
    assert_eq!(a.inner_edges(400, C.1 as i32, Q), None, "one past the bottom");
    let framed = window();
    let right = (framed.cx + C.0) as i32 - 2;
    assert_eq!(framed.inner_edges(right, (framed.cy + C.1 / 2) as i32, Q), None, "its frame's");
}

/// Only a window whose client draws its own frame gets the band: another
/// bare one (full screen) stays the client's, and none lights a control of
/// a frame NØNOS did not draw.
#[test]
fn the_band_is_only_for_a_window_that_draws_its_own_frame() {
    let pick = POINTER.split("\nfn edges(").nth(1).expect("edges");
    assert!(pick.contains("true => a.edges(x, y, content, q),"));
    assert!(pick.contains("false if scene.win.frame.own_frame => a.inner_edges(x, y, q),"));
    assert!(pick.contains("false => None,"));
    assert!(POINTER.contains("let hit = match x >= 0 && y >= 0 && a.framed() {"));
    const RESIZE: &str =
        include_str!("../../../../capsule_linux/src/linux/wayland/toplevel/resize.rs");
    let shape = RESIZE.split("pub fn shape(").nth(1).expect("shape");
    assert!(shape.contains("if !own_size(scene) || x < 0 || y < 0 {"));
    assert!(shape.contains("false if scene.win.frame.own_frame => {"));
}
