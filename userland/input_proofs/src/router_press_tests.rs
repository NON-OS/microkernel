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

//! A press and its release go to one receiver: the window pressed, or the
//! shell when the press was on the desk, whatever the pointer crosses, other
//! buttons pressed meanwhile, or a grab taken while the button is down.

use nonos_libc::{
    INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP, INPUT_KIND_POINTER_ABS, INPUT_KIND_WHEEL,
};

use crate::router_desk::*;

const A: u32 = 20;
const B: u32 = 21;
/// The kinds the desktop shell grabs for a drag, a menu or a dialog
/// (capsule_desktop_shell state/grab_rule.rs POINTER_BITS).
const SHELL_POINTER_GRAB: u32 = (1 << 3) | (1 << 4) | (1 << 5) | (1 << 6) | (1 << 7);

fn buttons(rx: &[Rx]) -> Vec<(u32, u16, u32)> {
    rx.iter()
        .filter(|r| r.kind == INPUT_KIND_BUTTON_DOWN || r.kind == INPUT_KIND_BUTTON_UP)
        .map(|r| (r.pid, r.kind, r.code))
        .collect()
}

#[test]
fn a_tap_on_the_desk_gives_the_shell_its_release_before_any_grab() {
    // A touchpad tap posts its press and release back to back, so both are
    // routed before the shell has read the press and grabbed the pointer for
    // an icon drag. The release must still be the shell's, or the icon stays
    // glued to the pointer and never opens.
    let mut ctx = desk(800, 600);
    point(&mut ctx, 100, 100);
    down(&mut ctx, LEFT);
    up(&mut ctx, LEFT);
    assert_eq!(
        buttons(&sent()),
        [(SHELL, INPUT_KIND_BUTTON_DOWN, LEFT), (SHELL, INPUT_KIND_BUTTON_UP, LEFT)]
    );
}

#[test]
fn a_press_on_the_desk_released_over_a_window_is_the_shells_release() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (300, 0, 300, 300));
    point(&mut ctx, 100, 100);
    down(&mut ctx, LEFT);
    point(&mut ctx, 350, 100);
    up(&mut ctx, LEFT);
    let rx = sent();
    assert_eq!(
        buttons(&rx),
        [(SHELL, INPUT_KIND_BUTTON_DOWN, LEFT), (SHELL, INPUT_KIND_BUTTON_UP, LEFT)]
    );
    assert!(rx.iter().all(|r| r.pid != A), "A saw the shell's press: {rx:?}");
}

#[test]
fn a_second_button_during_a_press_stays_with_the_pressed_window() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 300, 300));
    window(&mut ctx, B, (400, 0, 300, 300));
    point(&mut ctx, 100, 100);
    down(&mut ctx, LEFT);
    point(&mut ctx, 500, 100);
    down(&mut ctx, RIGHT);
    up(&mut ctx, RIGHT);
    up(&mut ctx, LEFT);
    let rx = sent();
    assert_eq!(
        buttons(&rx),
        [
            (A, INPUT_KIND_BUTTON_DOWN, LEFT),
            (A, INPUT_KIND_BUTTON_DOWN, RIGHT),
            (A, INPUT_KIND_BUTTON_UP, RIGHT),
            (A, INPUT_KIND_BUTTON_UP, LEFT),
        ]
    );
    assert!(rx.iter().all(|r| r.pid != B), "B saw part of A's press: {rx:?}");
}

#[test]
fn a_press_whose_release_was_lost_gives_way_to_the_next_press() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 300, 300));
    window(&mut ctx, B, (400, 0, 300, 300));
    point(&mut ctx, 100, 100);
    down(&mut ctx, LEFT);
    point(&mut ctx, 500, 100);
    // The left button goes down again with no release between: that release
    // was lost, and this press is B's.
    down(&mut ctx, LEFT);
    up(&mut ctx, LEFT);
    assert_eq!(
        buttons(&sent()),
        [
            (A, INPUT_KIND_BUTTON_DOWN, LEFT),
            (B, INPUT_KIND_BUTTON_DOWN, LEFT),
            (B, INPUT_KIND_BUTTON_UP, LEFT),
        ]
    );
}

#[test]
fn a_grab_taken_mid_press_still_gives_the_pressed_window_its_release() {
    // A dialog that comes up while a window's title bar is held grabs the
    // pointer for the shell. The window must still see its release, or its
    // drag never ends and it follows the pointer once the dialog is gone.
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 300, 300));
    point(&mut ctx, 100, 100);
    down(&mut ctx, LEFT);
    assert!(ctx.grabs.request(SHELL, SHELL_POINTER_GRAB));
    up(&mut ctx, LEFT);
    let rx = sent();
    assert!(
        buttons(&rx).contains(&(A, INPUT_KIND_BUTTON_UP, LEFT)),
        "A never saw its release: {rx:?}"
    );
    ctx.grabs.release(SHELL);
    point(&mut ctx, 600, 500);
    let rx = sent();
    assert!(
        rx.iter().all(|r| !(r.pid == A && r.kind == INPUT_KIND_POINTER_ABS)),
        "A still has the pointer after its release: {rx:?}"
    );
}

#[test]
fn after_an_icon_drag_the_wheel_reaches_the_window_under_the_pointer() {
    // The shell grabs the pointer for an icon drag after its press, so the
    // release arrives through the grab. That release must end the shell's
    // press too, or the wheel over a window goes on to the shell.
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (400, 0, 300, 300));
    point(&mut ctx, 100, 100);
    down(&mut ctx, LEFT);
    assert!(ctx.grabs.request(SHELL, SHELL_POINTER_GRAB));
    point(&mut ctx, 150, 150);
    up(&mut ctx, LEFT);
    ctx.grabs.release(SHELL);
    point(&mut ctx, 500, 100);
    sent();
    wheel(&mut ctx, 1);
    let rx = sent();
    assert_eq!(
        rx.iter().filter(|r| r.kind == INPUT_KIND_WHEEL).map(|r| r.pid).collect::<Vec<_>>(),
        [A]
    );
}

#[test]
fn the_wheel_turned_with_a_button_held_keeps_its_step() {
    // Scrolling while a selection is held extends it; a step the press grab
    // zeroed scrolled nothing.
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 300, 300));
    point(&mut ctx, 100, 100);
    down(&mut ctx, LEFT);
    sent();
    wheel(&mut ctx, -3);
    let rx = sent();
    let wheel: Vec<_> =
        rx.iter().filter(|r| r.kind == INPUT_KIND_WHEEL).map(|r| (r.pid, r.dy)).collect();
    assert_eq!(wheel, [(A, -3)]);
}
