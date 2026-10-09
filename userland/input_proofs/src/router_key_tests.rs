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

//! Every key a window was given goes down and comes up in that window, in
//! that order, whatever focus does while the key is held.

use nonos_libc::{INPUT_KIND_KEY_DOWN, INPUT_KIND_KEY_UP};

use crate::clients::world;
use crate::router_desk::*;

const A: u32 = 20;
const B: u32 = 21;
const X: u32 = b'x' as u32;

fn keys(rx: &[Rx]) -> Vec<(u32, u16, u32)> {
    rx.iter()
        .filter(|r| r.kind == INPUT_KIND_KEY_DOWN || r.kind == INPUT_KIND_KEY_UP)
        .map(|r| (r.pid, r.kind, r.code))
        .collect()
}

#[test]
fn a_held_key_repeating_into_a_new_focus_is_released_in_the_old_one_first() {
    // The keyboard repeats a held key as more presses. When focus moves while
    // it is held, the repeats go to the new window, and the window the key
    // went down in must see it come up: a Linux app under the Wayland bridge
    // repeats a key it holds on its own, forever.
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 300, 300));
    window(&mut ctx, B, (400, 0, 300, 300));
    world::set_focus(A);
    key_down(&mut ctx, X, 0);
    world::set_focus(B);
    key_down(&mut ctx, X, 0);
    key_down(&mut ctx, X, 0);
    key_up(&mut ctx, X, 0);
    assert_eq!(
        keys(&sent()),
        [
            (A, INPUT_KIND_KEY_DOWN, X),
            (A, INPUT_KIND_KEY_UP, X),
            (B, INPUT_KIND_KEY_DOWN, X),
            (B, INPUT_KIND_KEY_DOWN, X),
            (B, INPUT_KIND_KEY_UP, X),
        ]
    );
}

#[test]
fn a_repeat_in_the_same_window_is_not_released_between() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 300, 300));
    key_down(&mut ctx, X, 0);
    key_down(&mut ctx, X, 0);
    key_up(&mut ctx, X, 0);
    assert_eq!(
        keys(&sent()),
        [(A, INPUT_KIND_KEY_DOWN, X), (A, INPUT_KIND_KEY_DOWN, X), (A, INPUT_KIND_KEY_UP, X)]
    );
}
