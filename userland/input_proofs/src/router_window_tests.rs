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

//! One owner, two windows: each pointer event the router routes to a window
//! names that window in its frame, with x and y local to it, so a client with
//! a dialog over its main window (a Linux guest behind the Wayland bridge)
//! tells which of its surfaces the pointer is in.

use nonos_libc::{INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP, INPUT_KIND_POINTER_ABS};
use nonos_libc::{INPUT_KIND_KEY_DOWN, INPUT_KIND_WHEEL};

use crate::clients::world;
use crate::router_desk::*;
use crate::state::Context;

const GUEST: u32 = 30;
const MAIN: u32 = 1;
const DIALOG: u32 = 2;

/// The guest's main window, and a dialog of its own over part of it.
fn guest_desk() -> Context {
    let mut ctx = desk(800, 600);
    world::open_window(GUEST, MAIN, (100, 100, 400, 300));
    world::open_window(GUEST, DIALOG, (200, 150, 120, 80));
    subscribe(&mut ctx, GUEST);
    ctx
}

fn to_guest(rx: &[Rx], kind: u16) -> Vec<(Option<u32>, i32, i32)> {
    rx.iter().filter(|r| r.pid == GUEST && r.kind == kind).map(|r| (r.window, r.x, r.y)).collect()
}

#[test]
fn a_press_in_each_window_names_that_window_in_its_own_frame() {
    let mut ctx = guest_desk();
    point(&mut ctx, 210, 160);
    sent();
    down(&mut ctx, LEFT);
    up(&mut ctx, LEFT);
    let rx = sent();
    assert_eq!(to_guest(&rx, INPUT_KIND_BUTTON_DOWN), [(Some(DIALOG), 10, 10)]);
    assert_eq!(to_guest(&rx, INPUT_KIND_BUTTON_UP), [(Some(DIALOG), 10, 10)]);
    point(&mut ctx, 110, 120);
    sent();
    down(&mut ctx, LEFT);
    up(&mut ctx, LEFT);
    let rx = sent();
    assert_eq!(to_guest(&rx, INPUT_KIND_BUTTON_DOWN), [(Some(MAIN), 10, 20)]);
    assert_eq!(to_guest(&rx, INPUT_KIND_BUTTON_UP), [(Some(MAIN), 10, 20)]);
}

#[test]
fn moving_between_two_windows_of_one_owner_leaves_the_first() {
    let mut ctx = guest_desk();
    for x in [110, 111, 110, 111] {
        point(&mut ctx, x, 120);
    }
    sent();
    nonos_libc::advance_ms(1_000);
    point(&mut ctx, 215, 165);
    let hover = to_guest(&sent(), INPUT_KIND_POINTER_ABS);
    assert_eq!(hover, [(Some(MAIN), -1, -1), (Some(DIALOG), 15, 15)]);
}

#[test]
fn a_drag_out_of_the_pressed_window_stays_that_windows() {
    let mut ctx = guest_desk();
    point(&mut ctx, 210, 160);
    down(&mut ctx, LEFT);
    sent();
    // Out of the dialog, over the main window: the grab keeps the gesture
    // the dialog's, in its frame.
    point(&mut ctx, 150, 120);
    up(&mut ctx, LEFT);
    let rx = sent();
    assert_eq!(to_guest(&rx, INPUT_KIND_POINTER_ABS), [(Some(DIALOG), -50, -30)]);
    assert_eq!(to_guest(&rx, INPUT_KIND_BUTTON_UP), [(Some(DIALOG), -50, -30)]);
}

#[test]
fn a_wheel_names_the_window_under_the_pointer() {
    let mut ctx = guest_desk();
    point(&mut ctx, 300, 350);
    sent();
    wheel(&mut ctx, -3);
    let rx = sent();
    let w: Vec<_> =
        rx.iter().filter(|r| r.kind == INPUT_KIND_WHEEL).map(|r| (r.window, r.dy)).collect();
    assert_eq!(w, [(Some(MAIN), -3)]);
}

#[test]
fn keys_and_the_shells_events_name_no_window() {
    let mut ctx = guest_desk();
    point(&mut ctx, 700, 500);
    down(&mut ctx, LEFT);
    up(&mut ctx, LEFT);
    key_down(&mut ctx, 30, 0);
    let rx = sent();
    assert!(rx.iter().filter(|r| r.pid == SHELL).all(|r| r.window.is_none()), "{rx:?}");
    let keys: Vec<_> = rx.iter().filter(|r| r.kind == INPUT_KIND_KEY_DOWN).collect();
    assert!(!keys.is_empty() && keys.iter().all(|r| r.window.is_none()), "{rx:?}");
}

#[test]
fn a_press_on_no_window_is_told_to_the_window_manager() {
    let mut ctx = guest_desk();
    point(&mut ctx, 700, 500);
    down(&mut ctx, LEFT);
    up(&mut ctx, LEFT);
    assert_eq!(world::with(|w| w.outside), 1);
    point(&mut ctx, 210, 160);
    down(&mut ctx, LEFT);
    up(&mut ctx, LEFT);
    assert_eq!(world::with(|w| w.outside), 1, "a press on a window is not outside");
}
