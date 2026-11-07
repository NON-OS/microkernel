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

//! A focus_set the compositor never took. The window manager's stack has the
//! pressed window on top; the screen still draws the other one there, and a
//! press on what is drawn goes to the window under it. Nothing sent the order
//! again, so it stayed that way until the next raise. The owed restack lifts
//! every process bottom first and the screen agrees with the stack again.

use super::super::desk::Desk;

const TERMINAL: u32 = 10;
const EDITOR: u32 = 20;
const FILES: u32 = 30;

#[test]
fn a_lost_raise_leaves_the_screen_disagreeing_until_the_restack() {
    let mut d = Desk::new(1280, 720);
    d.open(TERMINAL, (60, 60, 700, 400), 0);
    d.open(FILES, (200, 100, 600, 400), 0);
    d.open(EDITOR, (300, 150, 900, 520), 0);
    d.assert_consistent("three windows open");

    d.compositor_lost = true;
    d.press(100, 300);
    assert_eq!(d.focused(), TERMINAL);
    assert_eq!(d.hit(400, 300), TERMINAL, "the stack has the terminal on top");
    assert_eq!(d.shown_at(400, 300), EDITOR, "the screen does not: the raise was lost");

    d.compositor_lost = false;
    d.restack();
    assert_eq!(d.shown_at(400, 300), TERMINAL);
    d.assert_consistent("after the restack");
}

/// A window raised between a new window's window_open and its first scene
/// submit: the window manager stacks it over the new one, while the
/// compositor puts the new layer on top of every other. Presses on what is
/// drawn went to the window under it until the next raise. The app raises
/// its window once the layer is in (setup/announce.rs), and both agree.
#[test]
fn a_window_raised_while_another_opens_is_put_back_in_order_by_the_new_one() {
    let mut d = Desk::new(1280, 720);
    d.open(TERMINAL, (60, 60, 700, 400), 0);
    d.open_window(EDITOR, (300, 150, 900, 520), 0);
    d.press(100, 300);
    assert_eq!(d.focused(), TERMINAL);
    d.first_submit(EDITOR);
    assert_eq!(d.hit(400, 300), TERMINAL, "the stack has the terminal on top");
    assert_eq!(d.shown_at(400, 300), EDITOR, "the screen has the new window on top");

    d.window_raise(EDITOR);
    assert_eq!(d.shown_at(400, 300), EDITOR);
    assert_eq!(d.hit(400, 300), EDITOR);
    assert_eq!(d.focused(), TERMINAL, "a raise takes no focus");
    d.assert_consistent("after the new window's raise");
}

#[test]
fn an_app_raises_its_window_once_its_first_layer_is_in() {
    const ANNOUNCE: &str = include_str!("../../../../app_skeleton/src/setup/announce.rs");
    let submit = ANNOUNCE.find("submit_scene(peers, surface_handle").expect("the first submit");
    let raise = ANNOUNCE.find("wm::window_raise(peers.wm").expect("the raise");
    assert!(submit < raise, "raised after the layer is in, not before");
}
