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
//! A Bluetooth device paired by Just Works while the trusted band is held
//! (G-76): its keys and clicks reach neither the lock screen or prompt nor
//! any window, and a key it pressed before the band came up is still let go
//! where it went down.

use nonos_libc::mk_uptime_ms;

use crate::clients::world;
use crate::route::trusted_hold::told;
use crate::router_tests::router_bond_kit::*;
use crate::router_desk::*;

const A: u32 = 20;
const KEY_X: u32 = b'x' as u32;

fn desk_with_window() -> crate::state::Context {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 400, 400));
    world::set_focus(A);
    ctx
}

fn hold(ctx: &mut crate::state::Context) {
    ctx.trusted = told(SHELL, mk_uptime_ms());
}

#[test]
fn under_a_hold_its_keys_and_clicks_go_nowhere() {
    let mut ctx = desk_with_window();
    hold(&mut ctx);
    key_down(&mut ctx, KEY_X, JUST_WORKS);
    key_up(&mut ctx, KEY_X, JUST_WORKS);
    point(&mut ctx, 100, 100);
    post(&mut ctx, nonos_libc::INPUT_KIND_BUTTON_DOWN, LEFT, JUST_WORKS, 0, 0);
    let sent = out();
    assert_eq!(events(&sent), [(SHELL, 0)], "only the wired mouse's motion: {sent:?}");
    assert_eq!(notices(&sent), [SHELL]);
    key_down(&mut ctx, KEY_X, PASSKEY);
    assert_eq!(events(&out()), [(SHELL, KEY_X)], "an authenticated keyboard types");
}

#[test]
fn a_key_pressed_before_the_hold_is_let_go_in_its_window() {
    let mut ctx = desk_with_window();
    key_down(&mut ctx, KEY_X, JUST_WORKS);
    hold(&mut ctx);
    key_up(&mut ctx, KEY_X, JUST_WORKS);
    let sent = out();
    assert_eq!(events(&sent), [(A, KEY_X), (A, KEY_X)], "{sent:?}");
    assert_eq!(notices(&sent), []);
}
