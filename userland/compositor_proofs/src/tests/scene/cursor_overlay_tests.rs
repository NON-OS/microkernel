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
//! The pointer's shape: drawn from its hot spot inside the damaged box, the
//! A layer that never asked for a shape has no say in it. The shell's chrome
//! covers the whole display over every window and asks for none; it used to
//! give the arrow everywhere, so no window's resize arrows were ever seen
//! (booted, HANDOFF/ui/evidence/switch-17abb88b7e).

use super::frame_util::layer;
use crate::frame_pacer::cursor_shapes::{ARROW_CODE, EW_CODE};
use crate::requests::pointer::{cursor_shape, CURSOR_WITHDRAW};
use crate::state::cursor_pick::shape_at;
use crate::state::scene::SceneTable;

#[test]
fn cursor_overlay_that_never_asked_leaves_the_window_its_shape() {
    let (mut window, chrome) = (layer(40, 0, 0, 400, 300, 2), layer(9, 0, 0, 1920, 1080, 30));
    window.cursor = Some(EW_CODE);
    let stack = [window, chrome];
    assert_eq!(shape_at(stack.iter(), 3, 150), EW_CODE, "the window's edge, under the chrome");
    assert_eq!(shape_at(stack.iter(), 900, 900), ARROW_CODE, "over no window that asked");
}

#[test]
fn cursor_overlay_window_that_asked_for_the_arrow_keeps_it() {
    let (mut low, mut high) = (layer(40, 0, 0, 400, 300, 2), layer(41, 0, 0, 400, 300, 2));
    (low.cursor, high.cursor) = (Some(EW_CODE), Some(ARROW_CODE));
    let stack = [low, high];
    assert_eq!(shape_at(stack.iter(), 3, 150), ARROW_CODE, "the window on top said the arrow");
}

/// The chrome asks for the arrow over its own pixels and withdraws over its
/// transparent ones: a menu over a window's edge shows the arrow, and with
/// the menu gone the edge shows its own arrows again.
#[test]
fn cursor_overlay_asks_the_arrow_and_withdraws_it() {
    let body =
        [9u32, 0, CURSOR_WITHDRAW, 0].iter().flat_map(|w| w.to_le_bytes()).collect::<Vec<_>>();
    assert_eq!(cursor_shape(&body), Some((9, None)), "the withdraw is well formed");
    let mut scene = SceneTable::new();
    scene.submit(layer(40, 0, 0, 400, 300, 2)).unwrap();
    scene.submit(layer(9, 0, 0, 1920, 1080, 30)).unwrap();
    scene.set_cursor(40, 40, Some(EW_CODE)).unwrap();
    scene.set_cursor(9, 9, Some(ARROW_CODE)).unwrap();
    let at = |s: &SceneTable| {
        let (l, n) = s.z_sorted_snapshot();
        shape_at(l[..n].iter(), 3, 150)
    };
    assert_eq!(at(&scene), ARROW_CODE, "over the chrome's own pixels");
    scene.set_cursor(9, 9, None).unwrap();
    assert_eq!(at(&scene), EW_CODE, "withdrawn: the window's edge again");
}
