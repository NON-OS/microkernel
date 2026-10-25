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


//! Ctrl+Alt+T: the press and its release go to the shell whatever window has
//! focus, so no app can keep the Terminal from being opened; a window's grab
//! and the trusted hold still take it, as they take every key; T alone, or
//! with one of the two, is the focused window's.

use nonos_libc::{INPUT_KIND_KEY_DOWN, INPUT_KIND_KEY_UP};

use crate::clients::world;
use crate::route::chord::is_terminal_chord;
use crate::router_desk::*;

const A: u32 = 20;
const T: u32 = b't' as u32;

#[test]
fn only_ctrl_alt_t_pressed_is_the_chord() {
    assert!(is_terminal_chord(INPUT_KIND_KEY_DOWN, T, MOD_CTRL | MOD_ALT));
    assert!(is_terminal_chord(INPUT_KIND_KEY_DOWN, b'T' as u32, MOD_CTRL | MOD_ALT | 1));
    assert!(!is_terminal_chord(INPUT_KIND_KEY_UP, T, MOD_CTRL | MOD_ALT));
    assert!(!is_terminal_chord(INPUT_KIND_KEY_DOWN, T, MOD_CTRL));
    assert!(!is_terminal_chord(INPUT_KIND_KEY_DOWN, T, MOD_ALT));
    assert!(!is_terminal_chord(INPUT_KIND_KEY_DOWN, b'y' as u32, MOD_CTRL | MOD_ALT));
}

#[test]
fn the_focused_window_cannot_take_it() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 400, 400));
    world::set_focus(A);
    key_down(&mut ctx, T, MOD_CTRL | MOD_ALT);
    key_up(&mut ctx, T, MOD_CTRL | MOD_ALT);
    let rx = sent();
    assert_eq!(of_kind(&rx, INPUT_KIND_KEY_DOWN), [(SHELL, T)]);
    assert_eq!(of_kind(&rx, INPUT_KIND_KEY_UP), [(SHELL, T)], "the release follows");
}

#[test]
fn ctrl_t_alone_is_the_windows() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 400, 400));
    world::set_focus(A);
    key_down(&mut ctx, T, MOD_CTRL);
    assert_eq!(of_kind(&sent(), INPUT_KIND_KEY_DOWN), [(A, T)]);
}

#[test]
fn a_grab_keeps_it_like_any_key() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 400, 400));
    world::set_focus(A);
    assert!(ctx.grabs.request(A, 0xFF));
    key_down(&mut ctx, T, MOD_CTRL | MOD_ALT);
    assert_eq!(of_kind(&sent(), INPUT_KIND_KEY_DOWN), [(A, T)], "not reserved over a grab");
}
