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

//! Alt+Tab: the press and its release go to the shell whatever window has
//! focus, so no app can keep a person from switching away from it; a grab
//! and the trusted hold still take it, as they take every key; Tab alone, or
//! with Ctrl, is the focused window's.

use nonos_libc::{INPUT_KIND_KEY_DOWN, INPUT_KIND_KEY_UP};

use crate::clients::world;
use crate::route::chord::is_switch_chord;
use crate::router_desk::*;

const A: u32 = 20;
const TAB: u32 = 0x09;

#[test]
fn switch_chord_is_alt_tab_pressed() {
    assert!(is_switch_chord(INPUT_KIND_KEY_DOWN, TAB, MOD_ALT));
    assert!(is_switch_chord(INPUT_KIND_KEY_DOWN, TAB, MOD_ALT | 1), "Shift turns it back");
    assert!(!is_switch_chord(INPUT_KIND_KEY_UP, TAB, MOD_ALT));
    assert!(!is_switch_chord(INPUT_KIND_KEY_DOWN, TAB, 0));
    assert!(!is_switch_chord(INPUT_KIND_KEY_DOWN, TAB, MOD_CTRL));
    assert!(!is_switch_chord(INPUT_KIND_KEY_DOWN, TAB, MOD_CTRL | MOD_ALT));
    assert!(!is_switch_chord(INPUT_KIND_KEY_DOWN, b'q' as u32, MOD_ALT));
}

#[test]
fn switch_chord_never_reaches_the_window() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 400, 400));
    world::set_focus(A);
    key_down(&mut ctx, TAB, MOD_ALT);
    key_up(&mut ctx, TAB, MOD_ALT);
    let rx = sent();
    assert_eq!(of_kind(&rx, INPUT_KIND_KEY_DOWN), [(SHELL, TAB)]);
    assert_eq!(of_kind(&rx, INPUT_KIND_KEY_UP), [(SHELL, TAB)], "the release follows");
}

#[test]
fn switch_chord_leaves_tab_and_ctrl_tab_to_the_window() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 400, 400));
    world::set_focus(A);
    key_down(&mut ctx, TAB, 0);
    key_down(&mut ctx, TAB, MOD_CTRL);
    assert_eq!(of_kind(&sent(), INPUT_KIND_KEY_DOWN), [(A, TAB), (A, TAB)]);
}

#[test]
fn switch_chord_goes_to_a_grab_like_any_key() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 400, 400));
    world::set_focus(A);
    assert!(ctx.grabs.request(A, 0xFF));
    key_down(&mut ctx, TAB, MOD_ALT);
    assert_eq!(of_kind(&sent(), INPUT_KIND_KEY_DOWN), [(A, TAB)], "not reserved over a grab");
}
