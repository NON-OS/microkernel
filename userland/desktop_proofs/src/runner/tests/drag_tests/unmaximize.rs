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
//! A drag of a maximized window's title bar takes it out of full screen
//! once the pointer has moved past 4 pixels: back at the size it had, held
//! by the title bar at the same share of its width, and the drag goes on
//! moving it from there. The live drive found such a drag did nothing: each
//! step was clamped back to the full-screen rect.

use super::super::super::chrome::MENUBAR_H;
use super::super::super::desk::event;
use super::super::super::drag::{self, DragState, Grip, PointerAction};
use super::rig::moved;
use crate::input::InputKind;

/// A maximized window on a 1920x1080 screen, under the menu bar.
const FULL: (u32, u32) = (1920, 1080 - MENUBAR_H);
const AT: (u32, u32) = (0, MENUBAR_H);

fn step(s: &mut DragState, kind: InputKind, x: i32, y: i32, maximized: bool) -> PointerAction {
    let size = if maximized { FULL } else { (800, 600) };
    drag::handle(s, size, AT, maximized, &event(kind, x, y))
}

fn grip(a: PointerAction) -> Option<Grip> {
    match a {
        PointerAction::Unmaximize(g) => Some(g),
        _ => None,
    }
}

#[test]
fn a_shake_of_the_press_leaves_the_window_maximized() {
    let mut s = DragState::new();
    step(&mut s, InputKind::ButtonDown, 960, 12, true);
    assert!(s.active, "the title bar is held");
    for (x, y) in [(962, 12), (964, 16), (956, 8)] {
        let a = step(&mut s, InputKind::PointerAbs, x, y, true);
        assert!(matches!(a, PointerAction::Held), "{x},{y}: within 4 px, nothing");
    }
}

#[test]
fn past_the_threshold_the_window_leaves_full_screen_under_the_pointer() {
    let mut s = DragState::new();
    step(&mut s, InputKind::ButtonDown, 960, 12, true);
    let g = grip(step(&mut s, InputKind::PointerAbs, 1000, 40, true)).expect("left full screen");
    assert_eq!(g.local, (1000, 40), "the pointer in the press's frame");
    assert_eq!(g.pointer, (1000, 40 + MENUBAR_H), "and on the screen");
    assert_eq!(g.press, (960, 12));
    assert_eq!(g.width, FULL.0);
    // Pressed halfway across: an 800 px window is held halfway across too.
    assert_eq!(g.restored_at(800, MENUBAR_H), (600, 28 + MENUBAR_H));
}

#[test]
fn the_drag_goes_on_from_where_the_window_was_put() {
    let mut s = DragState::new();
    step(&mut s, InputKind::ButtonDown, 960, 12, true);
    let g = grip(step(&mut s, InputKind::PointerAbs, 1000, 40, true)).expect("left full screen");
    let at = g.restored_at(800, MENUBAR_H);
    s.rebase(g.local, at);
    let a = step(&mut s, InputKind::PointerAbs, 1030, 90, false);
    assert_eq!(moved(a), Some((at.0 + 30, at.1 + 50)), "moved by the pointer since");
    step(&mut s, InputKind::ButtonUp, 1030, 90, false);
    assert!(!s.active, "the release ends the drag");
}

#[test]
fn the_restored_window_stays_on_the_screen() {
    let g = |pointer, press| Grip { pointer, local: (0, 0), press, width: 1920 };
    // Held at the right end: put against the right edge, not past it.
    assert_eq!(g((1919, 30), (1919, 10)).restored_at(800, 34).0, 1919 - 799);
    assert_eq!(g((1919, 30), (1919, 10)).restored_at(800, 34).0 + 800, 1920);
    // Held at the left end and high up: at the left edge, under the bar.
    assert_eq!(g((0, 0), (0, 10)).restored_at(800, 34), (0, 34));
    // Wider than the screen: at its left edge.
    assert_eq!(g((500, 300), (500, 10)).restored_at(2400, 34).0, 0);
}

#[test]
fn no_grip_overflows_or_divides_by_zero() {
    let far = Grip {
        pointer: (u32::MAX, u32::MAX),
        local: (0, 0),
        press: (i32::MAX, i32::MAX),
        width: 1920,
    };
    let (x, y) = far.restored_at(u32::MAX, u32::MAX);
    assert_eq!((x, y), (0, u32::MAX));
    let none = Grip { pointer: (100, 100), local: (0, 0), press: (i32::MIN, i32::MIN), width: 0 };
    assert_eq!(none.restored_at(800, 34), (0, 100));
    let mut s = DragState::new();
    step(&mut s, InputKind::ButtonDown, 5, 12, true);
    let g = grip(step(&mut s, InputKind::PointerAbs, i32::MIN, i32::MAX, true)).expect("a grip");
    assert_eq!(g.pointer.0, 0, "a pointer left of the screen is at its edge");
}

#[test]
fn a_window_not_maximized_moves_as_before() {
    let mut s = DragState::new();
    step(&mut s, InputKind::ButtonDown, 300, 25, false);
    let a = step(&mut s, InputKind::PointerAbs, 302, 26, false);
    assert_eq!(moved(a), Some((2, MENUBAR_H + 1)), "every step moves, however small");
}

#[test]
fn the_runner_takes_the_window_out_of_full_screen_before_it_moves_it() {
    const INPUT: &str = include_str!("../../../../../app_skeleton/src/runner/inbox/input.rs");
    assert!(INPUT.contains("PointerAction::Unmaximize(g) => out.unmaximize = Some(g),"));
    const APPLY: &str = include_str!("../../../../../app_skeleton/src/runner/apply_drain.rs");
    let leave = APPLY.find("unmaximize(booted, peers, request_id, grip);").expect("left");
    let step = APPLY.find("apply_move(booted, peers, request_id, mx, my);").expect("moved");
    assert!(leave < step, "out of full screen first, then any move of the same drain");
    const OPS: &str =
        include_str!("../../../../../app_skeleton/src/runner/window_ops/unmaximize.rs");
    let opened =
        OPS.find("if !reopen(booted, peers, request_id, placement, false) {").expect("reopen");
    let told =
        OPS.find("wm::window_maximize(peers.wm, rid, booted.manifest.window_id, rect, false)");
    let rebased = OPS.find("booted.drag.rebase(grip.local, (x, y));").expect("rebased");
    let told = told.expect("the window manager told");
    assert!(opened < told && told < rebased, "drawn at its size, told, then dragged on");
    assert!(OPS.contains("booted.asks.full.person_chose();"), "the person's choice");
    let refused = OPS.find("booted.drag.active = false;").expect("a refused reopen ends the drag");
    assert!(opened < refused && refused < told, "and is not tried again on every step");
}
