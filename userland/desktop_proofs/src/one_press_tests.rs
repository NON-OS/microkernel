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

//! The reported case end to end: with the text editor open and focused, one
//! press on the terminal's title bar raises the terminal, focuses it and starts
//! its drag, and the drag carries it wherever the pointer goes, across where
//! its own tabs were, until the release. After every step what the screen
//! shows is a full frame's worth and is what a press there would hit.

use super::desk::Desk;
use crate::scene_raise::raise_by_pid;
use crate::z_order::raise;

const TERMINAL: u32 = 10;
const EDITOR: u32 = 20;
/// The terminal's tab strip with one tab: a pill, the new-tab chip and three
/// tool buttons (capsule_terminal's nominal_w).
const TABS_W: u32 = 160 + 28 + 3 * 30;

/// The editor opened second, over the terminal's lower half, and focused.
/// The terminal's title bar runs from y 70 to 110 on screen, its tabs at the
/// right of it from x 460 to 738 (y 77 to 103).
fn desk() -> Desk {
    let mut d = Desk::new(1280, 720);
    d.open(TERMINAL, (60, 60, 700, 400), TABS_W);
    d.open(EDITOR, (300, 150, 900, 520), 0);
    assert_eq!(d.focused(), EDITOR);
    assert_eq!(d.shown_at(400, 300), EDITOR, "the editor covers the terminal's lower half");
    d.assert_consistent("two windows open");
    d
}

#[test]
fn one_press_on_the_title_bar_raises_focuses_and_starts_the_drag() {
    let mut d = desk();
    d.press(200, 85);
    assert_eq!(d.focused(), TERMINAL, "the press focused the terminal");
    assert_eq!(d.shown_at(400, 300), TERMINAL, "the terminal is drawn over the editor");
    assert_eq!(d.hit(400, 300), TERMINAL, "and a press there goes to it");
    assert!(d.app(TERMINAL).drag.active, "the same press started the drag");
    d.assert_consistent("after the press");

    // Straight down by 150: the window follows the pointer one for one.
    for step in 1..=15u32 {
        d.motion(200, 85 + 10 * step);
    }
    assert_eq!((d.app(TERMINAL).x, d.app(TERMINAL).y), (60, 210));
    d.assert_consistent("dragged down");
    d.release(200, 235);
    assert!(!d.app(TERMINAL).drag.active);
    assert_eq!(d.hit(200, 235), TERMINAL);
}

/// Dragging right runs the pointer, in the coordinates of the window as it was
/// at the press, across where the terminal's tabs were. Those steps went to
/// the tabs before: the window stopped following, and the release, landing
/// there too, never ended the drag.
#[test]
fn a_drag_across_the_tabs_keeps_moving_and_the_release_there_ends_it() {
    let mut d = desk();
    d.press(200, 85);
    for step in 1..=30u32 {
        d.motion(200 + 10 * step, 85);
    }
    let t = d.app(TERMINAL);
    assert_eq!((t.x, t.y), (360, 60), "the window followed all three hundred pixels");
    assert_eq!(t.accessory_events, 0, "no drag step was handed to the tabs");
    d.release(500, 85);
    assert!(!d.app(TERMINAL).drag.active, "the release over the tabs ended the drag");

    // With the button up the pointer only hovers; the window stays put.
    let before = (d.app(TERMINAL).x, d.app(TERMINAL).y);
    for step in 1..=10u32 {
        d.motion(500 + 7 * step, 85 + 3 * step);
    }
    assert_eq!((d.app(TERMINAL).x, d.app(TERMINAL).y), before);
    d.assert_consistent("after the drag");
}

#[test]
fn a_press_on_the_tabs_goes_to_the_tabs_and_moves_nothing() {
    let mut d = desk();
    d.press(500, 85);
    d.motion(300, 120);
    d.release(300, 120);
    let t = d.app(TERMINAL);
    assert_eq!((t.x, t.y), (60, 60));
    assert_eq!(t.accessory_events, 3, "the press, the motion off it and the release");
    assert_eq!(d.focused(), TERMINAL, "a press on the tabs still focuses the window");
}

#[test]
fn a_press_on_the_desktop_reaches_the_shell_and_moves_no_window() {
    let mut d = desk();
    d.press(20, 700);
    d.motion(40, 690);
    d.release(40, 690);
    assert_eq!(d.shell_presses, 1);
    assert_eq!(d.focused(), EDITOR);
    assert_eq!(d.app(TERMINAL).moves + d.app(EDITOR).moves, 0);
}

const SHELL: u32 = 5;
const DOCK_WINDOW_ID: u32 = 0x5442_4152;

/// The shell on a 1280 by 800 screen: its dock 64 high, 16 above the bottom
/// edge; and the editor as fit_to_display sizes and centres it there, 663 high
/// from y 91, over the dock's top 34 rows.
fn desk_with_dock() -> Desk {
    let mut d = Desk::new(1280, 800);
    d.open_shell(SHELL, (440, 720, 400, 64));
    d.open(EDITOR, (77, 91, 1126, 663), 0);
    d
}

/// The dock is drawn in the shell's chrome band, over every window, so where
/// a window reaches down across it the dock shows, and a press there goes to
/// the dock, not to the window drawn under it. Above the dock the window
/// keeps its presses.
#[test]
fn the_dock_draws_over_a_window_and_takes_the_presses_there() {
    let mut d = desk_with_dock();
    d.assert_consistent("a window over the dock's top edge");
    assert_eq!(d.shown_at(640, 740), SHELL, "the dock is drawn over the editor's lower edge");
    d.press(640, 740);
    assert_eq!(d.shell_presses, 1, "and a press there reaches the dock");
    d.press(640, 700);
    assert_eq!(d.shell_presses, 1, "above the dock the editor keeps its presses");
    assert_eq!(d.focused(), EDITOR);
}

/// A raise of the dock's window, or of the shell's layers, changes nothing:
/// the window manager ranks the shell's popup over every window whatever its
/// z, and the compositor keeps each of the shell's layers in its own band. So
/// what is shown and what is hit stay one, and an application raised after it
/// still never covers the dock.
#[test]
fn no_raise_moves_the_dock_under_a_window() {
    let mut d = desk_with_dock();
    let _ = raise(&mut d.windows, &mut d.z, SHELL, DOCK_WINDOW_ID);
    let _ = raise_by_pid(&mut d.scene, SHELL);
    d.press(640, 300);
    assert_eq!(d.focused(), EDITOR);
    d.assert_consistent("after the shell and the editor were raised");
    assert_eq!(d.shown_at(640, 740), SHELL);
    assert_eq!(d.hit(640, 740), SHELL);
    assert_eq!(d.shown_at(640, 300), EDITOR);
    assert_eq!(d.hit(640, 300), EDITOR);
}

/// Three windows and every press anywhere, then a drag of whatever was hit:
/// over long random sessions the screen never shows a stale pixel, and never
/// shows one window where a press would land on another.
#[test]
fn random_presses_and_drags_keep_what_is_shown_and_what_is_hit_the_same() {
    let mut seed = 0x9E37_79B9_7F4A_7C15u64;
    let mut below = |n: u32| {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        (seed % n as u64) as u32
    };
    for session in 0..12 {
        let mut d = Desk::new(640, 400);
        for pid in 1..=4u32 {
            let (w, h) = (120 + below(260), 90 + below(200));
            let acc = if below(2) == 0 { 0 } else { 60 + below(80) };
            d.open(pid, (below(640 - w), 46 + below(354 - h), w, h), acc);
        }
        for step in 0..40 {
            let (x, y) = (below(640), below(400));
            d.press(x, y);
            let (mut px, mut py) = (x as i32, y as i32);
            for _ in 0..below(6) {
                px = (px + below(81) as i32 - 40).clamp(0, 639);
                py = (py + below(81) as i32 - 40).clamp(0, 399);
                d.motion(px as u32, py as u32);
            }
            d.release(px as u32, py as u32);
            d.assert_consistent(&format!("session {session}, step {step}"));
        }
    }
}
