// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The recency rule runs when a grab is asked for, and no later: the shell
//! asking again after the user typed into a window is refused, and a press
//! the shell took under its own grab counts as the shell's.

use crate::clients::world;
use crate::route::grab_rule::grab_allowed;
use crate::router_desk::*;

const A: u32 = 20;
const ESC: u32 = 0x1B;
const MENU_GRAB: u32 = 1 | (1 << 3) | (1 << 4) | (1 << 5) | (1 << 6) | (1 << 7);

#[test]
fn asking_again_after_typing_into_the_window_is_refused() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 300, 300));
    point(&mut ctx, 600, 400);
    down(&mut ctx, LEFT);
    up(&mut ctx, LEFT);
    assert!(grab_allowed(&mut ctx, SHELL, true) && ctx.grabs.request(SHELL, MENU_GRAB));
    key_down(&mut ctx, ESC, 0);
    ctx.grabs.release(SHELL);
    key_down(&mut ctx, u32::from(b'x'), 0);
    assert!(!grab_allowed(&mut ctx, SHELL, true));
}

#[test]
fn a_press_under_the_shells_own_grab_is_the_shells() {
    for pressed in [true, false] {
        let mut ctx = desk(800, 600);
        assert!(grab_allowed(&mut ctx, SHELL, true) && ctx.grabs.request(SHELL, MENU_GRAB));
        if pressed {
            point(&mut ctx, 600, 400);
            down(&mut ctx, LEFT);
            up(&mut ctx, LEFT);
        }
        world::open(A, 0, 0, 300, 300);
        ctx.grabs.release(SHELL);
        assert_eq!(grab_allowed(&mut ctx, SHELL, true), pressed, "pressed {pressed}");
    }
}

#[test]
fn a_button_pressed_during_a_held_press_goes_with_that_press() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 300, 300));
    point(&mut ctx, 600, 400);
    down(&mut ctx, LEFT);
    key_down(&mut ctx, u32::from(b'x'), 0);
    assert!(!grab_allowed(&mut ctx, SHELL, true), "typed into the window");
    down(&mut ctx, RIGHT);
    assert!(grab_allowed(&mut ctx, SHELL, true), "the right button, in the shell's press");
}
