// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The shell grabs over a focused window only while the user's last press or
//! key went to the shell, by the router's own record: its menu, opened on
//! that press, then takes Esc and the click that closes it, and the window
//! keeps focus. The next press or key the router delivers to a window ends
//! that, motion does not, and input under the shell's own grab counts.

use nonos_libc::INPUT_KIND_BUTTON_DOWN as BUTTON_DOWN;
use nonos_libc::INPUT_KIND_KEY_DOWN as KEY_DOWN;

use crate::clients::wm::query_focus;
use crate::route::grab_rule::grab_allowed;
use crate::router_desk::*;
use crate::state::Context;

const A: u32 = 20;
const ESC: u32 = 0x1B;
/// The shell's menu grab: keys and the pointer (state/grab_rule.rs).
const MENU_GRAB: u32 = 1 | (1 << 3) | (1 << 4) | (1 << 5) | (1 << 6) | (1 << 7);

/// A focused window, and a click on the shell's own surface beside it.
fn menu_over_window(ctx: &mut Context) {
    window(ctx, A, (0, 0, 300, 300));
    point(ctx, 600, 400);
    down(ctx, LEFT);
    up(ctx, LEFT);
}

#[test]
fn a_press_on_the_shell_lets_it_grab_and_the_window_keeps_focus() {
    let mut ctx = desk(800, 600);
    menu_over_window(&mut ctx);
    assert!(grab_allowed(&mut ctx, SHELL, true));
    assert_eq!(query_focus(&mut 0, 0), Some(A));
}

#[test]
fn esc_and_a_click_into_the_window_then_reach_the_shell() {
    let mut ctx = desk(800, 600);
    menu_over_window(&mut ctx);
    assert!(grab_allowed(&mut ctx, SHELL, true) && ctx.grabs.request(SHELL, MENU_GRAB));
    sent();
    key_down(&mut ctx, ESC, 0);
    point(&mut ctx, 100, 100);
    down(&mut ctx, LEFT);
    let rx = sent();
    assert_eq!(of_kind(&rx, KEY_DOWN), [(SHELL, ESC)], "not the focused window's");
    assert_eq!(of_kind(&rx, BUTTON_DOWN), [(SHELL, LEFT)], "the click that closes it");
    assert_eq!(query_focus(&mut 0, 0), Some(A));
}

#[test]
fn a_key_or_a_press_into_the_window_ends_it_and_motion_does_not() {
    let mut ctx = desk(800, 600);
    menu_over_window(&mut ctx);
    point(&mut ctx, 100, 100);
    assert!(grab_allowed(&mut ctx, SHELL, true), "the pointer only passed over");
    key_down(&mut ctx, u32::from(b'x'), 0);
    assert!(!grab_allowed(&mut ctx, SHELL, true), "typed into the window");
    let mut ctx = desk(800, 600);
    menu_over_window(&mut ctx);
    point(&mut ctx, 100, 100);
    down(&mut ctx, LEFT);
    assert!(!grab_allowed(&mut ctx, SHELL, true), "pressed in the window");
}
