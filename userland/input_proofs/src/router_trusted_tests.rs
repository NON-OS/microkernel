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

//! The trusted path through the whole router: while the shell holds the
//! trusted band, every key, click and wheel goes to it alone, over a focused
//! window and over a grab, and a key held when the prompt came up is let go
//! where it went down. How a hold ends: router_hold_end_tests.

use nonos_libc::{mk_uptime_ms, INPUT_KIND_BUTTON_DOWN, INPUT_KIND_KEY_DOWN, INPUT_KIND_KEY_UP};

use crate::clients::world;
use crate::route::trusted_hold::told;
use crate::router_desk::*;

pub const A: u32 = 20;
pub const KEY_X: u32 = b'x' as u32;

pub fn held_desk() -> crate::state::Context {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 400, 400));
    world::set_focus(A);
    ctx.trusted = told(SHELL, mk_uptime_ms());
    ctx
}

#[test]
fn while_held_the_focused_window_gets_nothing() {
    let mut ctx = held_desk();
    key_down(&mut ctx, KEY_X, 0);
    key_up(&mut ctx, KEY_X, 0);
    point(&mut ctx, 100, 100);
    down(&mut ctx, LEFT);
    up(&mut ctx, LEFT);
    wheel(&mut ctx, 1);
    let rx = sent();
    assert!(rx.iter().all(|r| r.pid == SHELL), "a window heard input under a prompt: {rx:?}");
    assert_eq!(of_kind(&rx, INPUT_KIND_KEY_DOWN), [(SHELL, KEY_X)]);
    assert_eq!(of_kind(&rx, INPUT_KIND_BUTTON_DOWN), [(SHELL, LEFT)]);
}

#[test]
fn a_grab_is_overridden_and_none_is_granted() {
    let mut ctx = held_desk();
    assert!(ctx.grabs.request(A, 0xFF), "a grab taken before the prompt");
    key_down(&mut ctx, KEY_X, 0);
    down(&mut ctx, LEFT);
    assert!(sent().iter().all(|r| r.pid == SHELL), "the grab's holder heard the prompt's input");
}

#[test]
fn a_key_held_when_the_prompt_came_up_is_let_go_where_it_went_down() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 400, 400));
    world::set_focus(A);
    key_down(&mut ctx, KEY_X, 0);
    ctx.trusted = told(SHELL, mk_uptime_ms());
    key_up(&mut ctx, KEY_X, 0);
    key_down(&mut ctx, KEY_X, 0);
    let rx = sent();
    assert_eq!(of_kind(&rx, INPUT_KIND_KEY_UP), [(A, KEY_X)]);
    assert_eq!(of_kind(&rx, INPUT_KIND_KEY_DOWN), [(A, KEY_X), (SHELL, KEY_X)]);
}
