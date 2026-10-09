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

//! The router's routing, driven whole: keys to focus and their releases to
//! where their presses went, presses to the window under the pointer and the
//! rest of the gesture through the press grab, grabs, and peers that die.

use nonos_libc::{
    INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP, INPUT_KIND_KEY_DOWN, INPUT_KIND_KEY_UP,
    INPUT_KIND_POINTER_ABS,
};

use crate::clients::world;
use crate::router_desk::*;

const A: u32 = 20;
const B: u32 = 21;

#[test]
fn a_key_goes_to_focus_and_its_release_follows_its_press_across_a_focus_change() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 300, 300));
    window(&mut ctx, B, (400, 0, 300, 300));
    world::set_focus(A);
    key_down(&mut ctx, u32::from(b'x'), 0);
    world::set_focus(B);
    key_up(&mut ctx, u32::from(b'x'), 0);
    let rx = sent();
    assert_eq!(of_kind(&rx, INPUT_KIND_KEY_DOWN), [(A, u32::from(b'x'))]);
    assert_eq!(of_kind(&rx, INPUT_KIND_KEY_UP), [(A, u32::from(b'x'))]);
}

#[test]
fn the_reserved_chord_goes_to_the_shell_whatever_has_focus() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 300, 300));
    key_down(&mut ctx, 0x1B, MOD_CTRL | MOD_ALT);
    key_up(&mut ctx, 0x1B, MOD_CTRL | MOD_ALT);
    let rx = sent();
    assert_eq!(of_kind(&rx, INPUT_KIND_KEY_DOWN), [(SHELL, 0x1B)]);
    assert_eq!(of_kind(&rx, INPUT_KIND_KEY_UP), [(SHELL, 0x1B)]);
}

#[test]
fn a_release_off_the_pressed_window_still_reaches_it_in_its_own_frame() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (100, 100, 200, 200));
    window(&mut ctx, B, (500, 100, 200, 200));
    point(&mut ctx, 150, 150);
    down(&mut ctx, LEFT);
    point(&mut ctx, 600, 150);
    up(&mut ctx, LEFT);
    let rx = sent();
    let a: Vec<_> = rx.iter().filter(|r| r.pid == A).map(|r| (r.kind, r.x, r.y)).collect();
    assert_eq!(
        a,
        [
            (INPUT_KIND_BUTTON_DOWN, 50, 50),
            (INPUT_KIND_POINTER_ABS, 500, 50),
            (INPUT_KIND_BUTTON_UP, 500, 50),
        ]
    );
    assert!(rx.iter().all(|r| r.pid != B), "B saw part of A's press: {rx:?}");
}

#[test]
fn a_grab_whose_holder_died_lets_go_and_input_routes_again() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 300, 300));
    assert!(ctx.grabs.request(SHELL, 1 << INPUT_KIND_KEY_DOWN));
    nonos_libc::kill(SHELL);
    key_down(&mut ctx, u32::from(b'q'), 0);
    key_down(&mut ctx, u32::from(b'w'), 0);
    assert_eq!(of_kind(&sent(), INPUT_KIND_KEY_DOWN), [(A, u32::from(b'w'))]);
}

#[test]
fn the_cursor_reaches_the_whole_display_after_it_changes_size() {
    // The compositor starts on the firmware framebuffer and moves to the
    // virtio-gpu display once its driver answers, at that display's size.
    // The router learnt the size once: on a larger display the cursor stopped
    // at the old edge and a tablet's range was spread over the old size.
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (1000, 700, 300, 300));
    point(&mut ctx, 100, 100);
    world::with(|w| w.display = (1600, 1200));
    nonos_libc::advance_ms(5_000);
    // The next motion learns the new size; the cursor keeps its place in
    // proportion, (200, 200) here.
    point(&mut ctx, 100, 100);
    assert_eq!((ctx.cursor_x, ctx.cursor_y), (200, 200));
    point(&mut ctx, 1100, 800);
    sent();
    down(&mut ctx, LEFT);
    let rx = sent();
    assert!(
        rx.iter().any(|r| r.pid == A && r.kind == INPUT_KIND_BUTTON_DOWN),
        "the press never reached the window past the old edge: {rx:?}"
    );
    assert_eq!((ctx.cursor_x, ctx.cursor_y), (1100, 800));
}
