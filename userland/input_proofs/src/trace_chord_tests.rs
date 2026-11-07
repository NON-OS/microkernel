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

//! Ctrl+Alt+Shift+F: the shell's frame trace toggle reaches the shell
//! whatever window has focus; with any of the three held off it is the
//! focused window's key.

use nonos_libc::{INPUT_KIND_KEY_DOWN, INPUT_KIND_KEY_UP};

use crate::clients::world;
use crate::route::chord::is_trace_chord;
use crate::router_desk::*;

const A: u32 = 20;
const F: u32 = b'f' as u32;
const SHIFT: u16 = 1;

#[test]
fn only_ctrl_alt_shift_f_pressed_is_the_chord() {
    let all = MOD_CTRL | MOD_ALT | SHIFT;
    assert!(is_trace_chord(INPUT_KIND_KEY_DOWN, F, all));
    assert!(is_trace_chord(INPUT_KIND_KEY_DOWN, b'F' as u32, all));
    assert!(!is_trace_chord(INPUT_KIND_KEY_UP, F, all));
    assert!(!is_trace_chord(INPUT_KIND_KEY_DOWN, F, MOD_CTRL | MOD_ALT));
    assert!(!is_trace_chord(INPUT_KIND_KEY_DOWN, F, MOD_CTRL | SHIFT));
    assert!(!is_trace_chord(INPUT_KIND_KEY_DOWN, F, MOD_ALT | SHIFT));
    assert!(!is_trace_chord(INPUT_KIND_KEY_DOWN, b'g' as u32, all));
}

#[test]
fn the_focused_window_cannot_take_it() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 400, 400));
    world::set_focus(A);
    key_down(&mut ctx, F, MOD_CTRL | MOD_ALT | SHIFT);
    key_up(&mut ctx, F, MOD_CTRL | MOD_ALT | SHIFT);
    let rx = sent();
    assert_eq!(of_kind(&rx, INPUT_KIND_KEY_DOWN), [(SHELL, F)]);
    assert_eq!(of_kind(&rx, INPUT_KIND_KEY_UP), [(SHELL, F)], "the release follows");
}

#[test]
fn ctrl_alt_f_alone_is_the_windows() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 400, 400));
    world::set_focus(A);
    key_down(&mut ctx, F, MOD_CTRL | MOD_ALT);
    assert_eq!(of_kind(&sent(), INPUT_KIND_KEY_DOWN), [(A, F)]);
}
