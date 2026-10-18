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

//! The secure attention chord, Ctrl+Alt+Delete: it reaches the shell alone,
//! press and release, whatever has focus and even over a window's grab, as
//! Ctrl+Alt+Esc now does too; Delete alone, or with one modifier, is the
//! focused window's like any key.

use nonos_libc::{INPUT_KIND_KEY_DOWN, INPUT_KIND_KEY_UP};

use crate::clients::world;
use crate::route::chord::{is_secure_attention, KEY_DELETE};
use crate::router_desk::*;

const A: u32 = 20;
const KEY_ESC: u32 = 0x1B;

#[test]
fn only_ctrl_alt_delete_pressed_is_the_chord() {
    assert_eq!(KEY_DELETE, crate::ps2_keycodes::KEYCODE_DEL);
    assert!(is_secure_attention(INPUT_KIND_KEY_DOWN, KEY_DELETE, MOD_CTRL | MOD_ALT));
    assert!(is_secure_attention(INPUT_KIND_KEY_DOWN, KEY_DELETE, MOD_CTRL | MOD_ALT | 1));
    assert!(!is_secure_attention(INPUT_KIND_KEY_UP, KEY_DELETE, MOD_CTRL | MOD_ALT));
    assert!(!is_secure_attention(INPUT_KIND_KEY_DOWN, KEY_DELETE, MOD_CTRL));
    assert!(!is_secure_attention(INPUT_KIND_KEY_DOWN, KEY_DELETE, MOD_ALT));
    assert!(!is_secure_attention(INPUT_KIND_KEY_DOWN, KEY_ESC, MOD_CTRL | MOD_ALT));
}

#[test]
fn the_chords_reach_the_shell_alone_even_over_a_grab() {
    for code in [KEY_DELETE, KEY_ESC] {
        let mut ctx = desk(800, 600);
        window(&mut ctx, A, (0, 0, 400, 400));
        world::set_focus(A);
        assert!(ctx.grabs.request(A, 0xFF));
        key_down(&mut ctx, code, MOD_CTRL | MOD_ALT);
        key_up(&mut ctx, code, MOD_CTRL | MOD_ALT);
        let rx = sent();
        assert_eq!(of_kind(&rx, INPUT_KIND_KEY_DOWN), [(SHELL, code)]);
        assert_eq!(of_kind(&rx, INPUT_KIND_KEY_UP), [(SHELL, code)], "the release follows");
        assert!(rx.iter().all(|r| r.pid == SHELL), "{code:#x} reached a window: {rx:?}");
    }
}

#[test]
fn delete_alone_is_the_windows() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 400, 400));
    world::set_focus(A);
    key_down(&mut ctx, KEY_DELETE, MOD_CTRL);
    assert_eq!(of_kind(&sent(), INPUT_KIND_KEY_DOWN), [(A, KEY_DELETE)]);
}
