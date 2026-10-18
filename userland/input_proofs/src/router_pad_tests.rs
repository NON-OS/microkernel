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

//! Gamepads follow focus exactly as keys do (the the maintainers' ruling on B-57): a
//! window subscribed to pad events but not focused sees none of them; a
//! button's release goes where its press went though focus moved; two pads
//! pressing the same button are two presses. Under a prompt:
//! router_pad_hold_tests.

use nonos_libc::InputEvent;

use crate::clients::world;
use crate::route::pad::{PAD_AXIS, PAD_BUTTON_DOWN, PAD_BUTTON_UP};
use crate::route::route_event;
use crate::router_desk::*;
use crate::state::Context;

const A: u32 = 20;
const B: u32 = 21;
pub const SOUTH: u32 = 0x130;
/// Every kind up to the pad axis, which the desk's default mask leaves out.
const WITH_PADS: u32 = 0x7FF;

pub fn pad(ctx: &mut Context, kind: u16, y: i32, code: u32) {
    let ev = InputEvent { kind, code, y, ..InputEvent::default() };
    route_event(ctx, &ev);
}

pub fn two_windows() -> Context {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 300, 300));
    window(&mut ctx, B, (400, 0, 300, 300));
    for pid in [SHELL, A, B] {
        assert!(ctx.subscriptions.upsert(pid, WITH_PADS));
    }
    world::set_focus(A);
    ctx
}

#[test]
fn an_unfocused_subscriber_sees_no_pad_event() {
    let mut ctx = two_windows();
    pad(&mut ctx, PAD_BUTTON_DOWN, 0, SOUTH);
    pad(&mut ctx, PAD_AXIS, 0, 1);
    pad(&mut ctx, PAD_BUTTON_UP, 0, SOUTH);
    let rx = sent();
    assert_eq!(rx.len(), 3, "{rx:?}");
    assert!(rx.iter().all(|r| r.pid == A), "a window without focus read the pad: {rx:?}");
}

#[test]
fn a_release_follows_its_press_and_pads_are_apart() {
    let mut ctx = two_windows();
    pad(&mut ctx, PAD_BUTTON_DOWN, 0, SOUTH);
    world::set_focus(B);
    pad(&mut ctx, PAD_BUTTON_DOWN, 1, SOUTH);
    pad(&mut ctx, PAD_BUTTON_UP, 0, SOUTH);
    pad(&mut ctx, PAD_BUTTON_UP, 1, SOUTH);
    let rx = sent();
    assert_eq!(of_kind(&rx, PAD_BUTTON_DOWN), [(A, SOUTH), (B, SOUTH)]);
    assert_eq!(of_kind(&rx, PAD_BUTTON_UP), [(A, SOUTH), (B, SOUTH)], "each to its press");
}
