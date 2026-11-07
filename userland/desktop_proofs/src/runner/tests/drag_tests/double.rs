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
//! A double click on the title bar maximizes the window, or restores it, as
//! its maximize control does: two presses there at most 400 ms and 4 pixels
//! apart. The live drive found a double click did nothing, as no native
//! frame handled one.

use super::super::super::desk::event;
use super::super::super::drag::{self, press_ms, DragState, PointerAction};
use super::rig::{moved, H, W};
use crate::input::{InputEvent, InputKind};

fn press_at(s: &mut DragState, now: Option<i64>, x: i32, y: i32, maximized: bool) -> PointerAction {
    s.clock_ms = now;
    drag::handle(s, (W, H), (100, 100), maximized, &event(InputKind::ButtonDown, x, y))
}

fn up(s: &mut DragState, x: i32, y: i32) {
    drag::handle(s, (W, H), (100, 100), false, &event(InputKind::ButtonUp, x, y));
}

fn toggles(a: &PointerAction) -> bool {
    matches!(a, PointerAction::ToggleMaximize)
}

#[test]
fn two_quick_presses_on_the_title_bar_maximize() {
    let mut s = DragState::new();
    assert!(!toggles(&press_at(&mut s, Some(10_000), 300, 25, false)));
    assert!(s.active, "the first press drags as ever");
    up(&mut s, 300, 25);
    let a = press_at(&mut s, Some(10_250), 302, 24, false);
    assert!(toggles(&a), "the second, 250 ms and 2 px from the first");
    assert!(!s.active && s.resizing.is_none(), "and starts no drag");
    let m = drag::handle(&mut s, (W, H), (100, 100), false, &event(InputKind::PointerAbs, 400, 80));
    assert!(moved(m).is_none(), "the pointer moving after it moves nothing");
}

#[test]
fn a_maximized_window_is_restored_the_same_way() {
    let mut s = DragState::new();
    press_at(&mut s, Some(5_000), 300, 18, true);
    up(&mut s, 300, 18);
    assert!(toggles(&press_at(&mut s, Some(5_100), 300, 18, true)));
}

#[test]
fn presses_too_slow_or_too_far_apart_each_drag() {
    for (gap, dx) in [(401, 0), (100, 5), (1_000, 40)] {
        let mut s = DragState::new();
        press_at(&mut s, Some(1_000), 300, 25, false);
        up(&mut s, 300, 25);
        let a = press_at(&mut s, Some(1_000 + gap), 300 + dx, 25, false);
        assert!(!toggles(&a) && s.active, "{gap} ms, {dx} px: a second drag");
    }
}

#[test]
fn a_press_that_dragged_is_not_half_a_double_click() {
    let mut s = DragState::new();
    press_at(&mut s, Some(2_000), 300, 25, false);
    let m = drag::handle(&mut s, (W, H), (100, 100), false, &event(InputKind::PointerAbs, 330, 40));
    assert!(moved(m).is_some());
    up(&mut s, 330, 40);
    assert!(!toggles(&press_at(&mut s, Some(2_150), 300, 25, false)));
}

#[test]
fn a_third_press_starts_again_and_a_press_elsewhere_forgets_the_first() {
    let mut s = DragState::new();
    press_at(&mut s, Some(0), 300, 25, false);
    up(&mut s, 300, 25);
    assert!(toggles(&press_at(&mut s, Some(100), 300, 25, false)));
    up(&mut s, 300, 25);
    assert!(!toggles(&press_at(&mut s, Some(200), 300, 25, false)), "the third is a first");
    up(&mut s, 300, 25);
    press_at(&mut s, Some(250), 300, 200, false);
    up(&mut s, 300, 200);
    assert!(!toggles(&press_at(&mut s, Some(300), 300, 25, false)), "content pressed between");
}

#[test]
fn no_clock_or_a_clock_gone_back_is_never_a_double_click() {
    let mut s = DragState::new();
    press_at(&mut s, None, 300, 25, false);
    up(&mut s, 300, 25);
    assert!(!toggles(&press_at(&mut s, None, 300, 25, false)), "the runner said no time");
    let mut s = DragState::new();
    press_at(&mut s, Some(9_000), 300, 25, false);
    up(&mut s, 300, 25);
    assert!(!toggles(&press_at(&mut s, Some(8_900), 300, 25, false)), "earlier than the first");
    let mut s = DragState::new();
    press_at(&mut s, Some(i64::MIN), 300, 25, false);
    up(&mut s, 300, 25);
    assert!(!toggles(&press_at(&mut s, Some(i64::MAX), 300, 25, false)), "no overflow");
}

#[test]
fn far_coordinates_never_overflow() {
    let mut s = DragState::new();
    s.clock_ms = Some(0);
    let far = InputEvent { x: i32::MAX, y: 25, ..event(InputKind::ButtonDown, 0, 0) };
    drag::handle(&mut s, (W, H), (100, 100), false, &far);
    assert!(!toggles(&press_at(&mut s, Some(10), i32::MIN + 1, 25, false)));
}

#[test]
fn the_runner_gives_the_clock_and_a_double_click_maximizes() {
    const INPUT: &str = include_str!("../../../../../app_skeleton/src/runner/inbox/input.rs");
    assert!(INPUT.contains("PointerAction::ToggleMaximize => return stop(&mut out.maximize),"));
    const DRAIN: &str = include_str!("../../../../../app_skeleton/src/runner/inbox/drain.rs");
    let at =
        DRAIN.find("drag_state.clock_ms = Some(press_ms(event.timestamp_ns, mk_uptime_ms()));");
    let at = at.expect("the clock, the event's own first");
    let call = DRAIN.find("if input(app, drag_state, &mut out, event,").expect("the input");
    assert!(at < call, "set before each routed event is handled");
    const APPLY: &str = include_str!("../../../../../app_skeleton/src/runner/apply_drain.rs");
    assert!(APPLY.contains("if result.maximize {\n        booted.asks.full.person_chose();\n        toggle(booted, peers, request_id);"));
}

#[test]
fn a_press_is_timed_by_its_event_when_it_carries_a_time() {
    assert_eq!(press_ms(10_250_000_000, 99_999), 10_250, "the router's time, not the read");
    assert_eq!(press_ms(999_999, 7), 0, "under a millisecond is 0 ms");
    assert_eq!(press_ms(0, 4_321), 4_321, "no time: when the runner read it");
    assert_eq!(press_ms(u64::MAX, 5), i64::try_from(u64::MAX / 1_000_000).unwrap_or(5));
}

#[test]
fn two_presses_250_ms_apart_read_900_ms_apart_still_maximize() {
    // A runner busy drawing reads the second press late; timed by the read,
    // the two were too far apart (live under TCG, two clicks 310 ms apart).
    let mut s = DragState::new();
    press_at(&mut s, Some(press_ms(10_000_000_000, 10_000)), 300, 25, false);
    up(&mut s, 300, 25);
    let late_read = 10_900;
    let a = press_at(&mut s, Some(press_ms(10_250_000_000, late_read)), 300, 25, false);
    assert!(toggles(&a));
    let mut s = DragState::new();
    press_at(&mut s, Some(10_000), 300, 25, false);
    up(&mut s, 300, 25);
    assert!(!toggles(&press_at(&mut s, Some(late_read), 300, 25, false)), "by the read: none");
}

fn motion(s: &mut DragState, x: i32, y: i32) -> PointerAction {
    drag::handle(s, (W, H), (100, 100), false, &event(InputKind::PointerAbs, x, y))
}

#[test]
fn a_tablet_click_with_its_motion_is_still_half_a_double_click() {
    // A tablet posts where the pointer is before each button change, so the
    // release of a click comes with motion that goes nowhere; that motion
    // cleared the first press, and on a usb-tablet guest no double click
    // on a title bar ever maximized.
    for (dx, dy) in [(0, 0), (3, -2), (-4, 4)] {
        let mut s = DragState::new();
        press_at(&mut s, Some(10_000), 300, 25, false);
        motion(&mut s, 300 + dx, 25 + dy);
        up(&mut s, 300 + dx, 25 + dy);
        motion(&mut s, 300, 25);
        let a = press_at(&mut s, Some(10_200), 300, 25, false);
        assert!(toggles(&a), "motion of ({dx}, {dy}) inside the reach");
    }
}

#[test]
fn motion_past_the_reach_while_held_is_a_drag() {
    for (dx, dy) in [(5, 0), (0, -5), (40, 30)] {
        let mut s = DragState::new();
        press_at(&mut s, Some(10_000), 300, 25, false);
        assert!(moved(motion(&mut s, 300 + dx, 25 + dy)).is_some(), "the window moves");
        motion(&mut s, 300, 25);
        up(&mut s, 300, 25);
        assert!(!toggles(&press_at(&mut s, Some(10_200), 300, 25, false)), "({dx}, {dy})");
    }
}
