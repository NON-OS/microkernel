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

//! The green button end to end: app_skeleton's full-screen rect, the window
//! manager's maximise, the compositor's scene, and the hit test, for every
//! app (they all go through maximize::toggle), at every display scale.

use super::chrome::full_screen;
use super::desk::{Desk, DOCK_WINDOW_ID};
use super::full_screen_ask::{FullScreenAsk, Turn};
use crate::window::full_screen::covers_dock_at;

const APP: u32 = 30;
const SIZES: [(u32, u32); 4] = [(1280, 800), (1920, 1080), (2560, 1600), (3840, 2160)];

/// Green fills the display from the foot of the menubar to the bottom edge,
/// full width, at every scale: the window's pixels are on the bottom row and
/// a press there goes to it.
#[test]
fn green_reaches_the_bottom_edge_at_every_scale() {
    for (w, h) in SIZES {
        let mut desk = Desk::new(w, h);
        desk.open(APP, (200, 150, 640, 480), 0);
        desk.green(APP);
        let a = desk.app(APP);
        let (_, bar, _, _) = full_screen(w, h);
        assert_eq!((a.x, a.y, a.w, a.y + a.h), (0, bar, w, h), "{w}x{h}");
        for x in [0, w / 2, w - 1] {
            assert_eq!(desk.shown_at(x, h - 1), APP, "{w}x{h}: bottom row at {x}");
            assert_eq!(desk.hit(x, h - 1), APP, "{w}x{h}: a press at the bottom");
        }
        assert!(covers_dock_at(&desk.windows, APP, 0x5749_4E00 | APP), "{w}x{h}");
        desk.assert_consistent("full screen");
    }
}

/// Green again puts back the rect the window had, and it covers the dock's
/// band no longer.
#[test]
fn green_again_restores_the_rect_the_window_had() {
    let mut desk = Desk::new(1920, 1080);
    desk.open(APP, (200, 150, 640, 480), 0);
    let before = {
        let a = desk.app(APP);
        (a.x, a.y, a.w, a.h)
    };
    desk.green(APP);
    desk.green(APP);
    let a = desk.app(APP);
    assert_eq!((a.x, a.y, a.w, a.h), before);
    assert!(!covers_dock_at(&desk.windows, APP, 0x5749_4E00 | APP));
    desk.assert_consistent("restored");
}

/// An app that asks from the start opens full screen; one that stops asking
/// gets its rect back.
#[test]
fn an_app_asking_from_its_first_frame_opens_full_screen_and_gives_it_back() {
    let mut ask = FullScreenAsk::new();
    assert_eq!(ask.step(true, false), Turn::Enter);
    assert_eq!(ask.step(true, true), Turn::Stay, "asked once, taken once");
    assert_eq!(ask.step(false, true), Turn::Leave);
    assert_eq!(ask.step(false, false), Turn::Stay);
}

/// Video playing asks; the person made the window full screen first with
/// green, so the end of playback leaves it as they chose.
#[test]
fn the_ask_never_undoes_what_the_person_chose() {
    let mut ask = FullScreenAsk::new();
    // Full screen by green, then the app asks and stops asking.
    assert_eq!(ask.step(true, true), Turn::Stay);
    assert_eq!(ask.step(false, true), Turn::Stay);

    // Taken by the ask, then green pressed twice by the person (out and back
    // in): the person's full screen stays when the ask ends.
    let mut ask = FullScreenAsk::new();
    assert_eq!(ask.step(true, false), Turn::Enter);
    ask.person_chose();
    assert_eq!(ask.step(false, true), Turn::Stay);

    // Taken by the ask, left by green: the ask ending changes nothing, and
    // asking again takes it again.
    let mut ask = FullScreenAsk::new();
    assert_eq!(ask.step(true, false), Turn::Enter);
    ask.person_chose();
    assert_eq!(ask.step(false, false), Turn::Stay);
    assert_eq!(ask.step(true, false), Turn::Enter);
}

// The dock over a full-screen window: the shell's rule (the real
// state/taskbar code) driven by the window manager's full-screen
// notifications and the pointer the router mirrors to it, its chrome
// composited over the window.

const SHELL: u32 = 5;
const OTHER: u32 = 31;
/// The dock on a 1280 by 800 screen: 400 by 64, 16 above the bottom edge.
const DOCK: (u32, u32, u32, u32) = (440, 720, 400, 64);

fn desk_full_screen() -> Desk {
    let mut d = Desk::new(1280, 800);
    d.open_shell(SHELL, DOCK);
    d.open(APP, (200, 150, 640, 480), 0);
    d.green(APP);
    d
}

/// Every pixel of the dock's area, panel and shadow, shows `pid`.
fn dock_area_shows(d: &Desk, pid: u32) -> bool {
    let (x, y, w, h) = DOCK;
    (y - 3..y + h + 3).all(|py| (x - 3..x + w + 3).all(|px| d.shown_at(px, py) == pid))
}

#[test]
fn the_dock_hides_while_a_full_screen_window_shows_and_its_band_is_the_window() {
    let mut d = desk_full_screen();
    assert!(!d.taskbar.visible);
    assert!(dock_area_shows(&d, APP), "no panel, no shadow, no band: the window");
    assert!(d.chrome.iter().all(|&c| c == 0), "nothing left on the chrome");
    assert!(d.windows.find(SHELL, DOCK_WINDOW_ID).is_none(), "the dock's window is closed");
    // The commit covered the shadow as well as the panel, so the compositor
    // drew the window there: a full recomposite matches the screen.
    let area = *d.dock_commits.last().expect("committed");
    assert!(area.x <= DOCK.0 - 3 && area.y <= DOCK.1 - 3);
    assert!(area.x + area.width >= DOCK.0 + DOCK.2 + 3 && area.y + area.height == 800);
    d.assert_consistent("dock hidden over the full-screen window");
}

#[test]
fn a_press_at_the_bottom_goes_to_the_app_while_hidden_and_to_the_dock_while_shown() {
    let mut d = desk_full_screen();
    assert_eq!(d.hit(640, 750), APP);
    d.press(640, 750);
    d.release(640, 750);
    assert_eq!(d.shell_presses, 0, "the app took it");

    d.motion(640, 799);
    assert_eq!(d.hit(640, 750), SHELL);
    d.press(640, 750);
    assert_eq!(d.shell_presses, 1, "the dock took it");
}

#[test]
fn touching_the_bottom_edge_shows_the_dock_over_the_window_and_leaving_hides_it() {
    let mut d = desk_full_screen();
    d.motion(640, 790);
    assert!(!d.taskbar.visible, "near the edge is not the edge");
    d.motion(100, 799);
    assert!(d.taskbar.visible, "the bottom edge, anywhere along it");
    assert_eq!(d.shown_at(640, 750), SHELL, "drawn over the window");
    assert!(d.windows.find(SHELL, DOCK_WINDOW_ID).is_some(), "its window open again");
    d.assert_consistent("dock revealed");

    d.motion(640, 730);
    assert!(d.taskbar.visible, "the pointer on the dock keeps it");
    d.motion(640, 400);
    assert!(!d.taskbar.visible, "leaving its area puts it away");
    assert!(dock_area_shows(&d, APP), "no residue");
    assert!(d.windows.find(SHELL, DOCK_WINDOW_ID).is_none());
    d.assert_consistent("dock put away");
}

#[test]
fn green_again_brings_the_dock_back() {
    let mut d = desk_full_screen();
    d.green(APP);
    assert!(d.taskbar.visible);
    assert_eq!(d.shown_at(640, 750), SHELL);
    assert_eq!(d.hit(640, 750), SHELL);
    d.assert_consistent("restored");
}

#[test]
fn closing_or_minimising_the_full_screen_window_brings_the_dock_back() {
    let mut d = desk_full_screen();
    d.minimize(APP);
    assert!(d.taskbar.visible, "minimised");
    assert_eq!(d.hit(640, 750), SHELL);
    d.assert_consistent("minimised");
    d.restore_window(APP);
    assert!(!d.taskbar.visible, "brought back full screen from the dock");
    d.assert_consistent("restored from the dock");

    d.close(APP);
    assert!(d.taskbar.visible, "closed");
    assert_eq!(d.shown_at(640, 750), SHELL);
    d.assert_consistent("closed");

    // A process that ends without its close: the shell forgets it once a
    // second (forget_dead_windows), and the dock comes back.
    let mut d = desk_full_screen();
    crate::taskbar::forget_dead_windows(&mut d.taskbar, |pid| pid != APP);
    d.shell_sync();
    assert!(d.taskbar.visible, "its process ended");
}

#[test]
fn another_window_opening_or_closing_keeps_full_screen() {
    let mut d = desk_full_screen();
    d.open(OTHER, (300, 200, 400, 300), 0);
    d.shell_sync();
    assert!(!d.taskbar.visible, "another window opened");
    assert!(covers_dock_at(&d.windows, APP, 0x5749_4E00 | APP));
    d.minimize(OTHER);
    assert!(!d.taskbar.visible, "another window minimised");
    d.restore_window(OTHER);
    d.close(OTHER);
    assert!(!d.taskbar.visible, "another window closed");
    assert_eq!(d.hit(640, 750), APP);
    d.assert_consistent("the other window gone");
}
