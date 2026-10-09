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

//! A desk for the router: a fresh router context over a display with the
//! shell up, devices that post events the way the drivers post them, and the
//! deliveries read back off the wire in order.

use nonos_libc::{
    InputEvent, INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP, INPUT_KIND_KEY_DOWN,
    INPUT_KIND_KEY_UP, INPUT_KIND_POINTER_REL, INPUT_KIND_WHEEL,
};

use crate::clients::world;
use crate::route::route_event;
use crate::state::Context;

pub const SHELL: u32 = 10;
pub const LEFT: u32 = 1;
pub const RIGHT: u32 = 2;
pub const MOD_CTRL: u16 = 1 << 1;
pub const MOD_ALT: u16 = 1 << 2;
const EVERY_KIND: u32 = 0xFF;

/// One delivery as the receiver decodes it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rx {
    pub pid: u32,
    pub kind: u16,
    pub flags: u16,
    pub code: u32,
    pub x: i32,
    pub y: i32,
    pub dy: i32,
}

/// A router over a `width` by `height` display, the shell subscribed, the
/// cursor at the centre, and nothing sent yet.
pub fn desk(width: u32, height: u32) -> Context {
    nonos_libc::reset();
    world::reset(width, height, SHELL);
    let mut ctx = Context::new();
    ctx.cursor.configure(width, height);
    subscribe(&mut ctx, SHELL);
    ctx
}

/// An app window for `pid`, on top with focus, subscribed to every kind.
pub fn window(ctx: &mut Context, pid: u32, r: (u32, u32, u32, u32)) {
    world::open(pid, r.0, r.1, r.2, r.3);
    subscribe(ctx, pid);
}

pub fn subscribe(ctx: &mut Context, pid: u32) {
    assert!(ctx.subscriptions.upsert(pid, EVERY_KIND));
}

pub fn post(ctx: &mut Context, kind: u16, code: u32, flags: u16, dx: i32, dy: i32) {
    let ev =
        InputEvent { kind, flags, code, x: 0, y: 0, delta_x: dx, delta_y: dy, timestamp_ns: 0 };
    route_event(ctx, &ev);
}

pub fn key_down(ctx: &mut Context, code: u32, flags: u16) {
    post(ctx, INPUT_KIND_KEY_DOWN, code, flags, 0, 0);
}

pub fn key_up(ctx: &mut Context, code: u32, flags: u16) {
    post(ctx, INPUT_KIND_KEY_UP, code, flags, 0, 0);
}

/// Relative motion that lands the cursor on (x, y) at the default
/// sensitivity, as a mouse moves it.
pub fn point(ctx: &mut Context, x: u32, y: u32) {
    let dx = x as i32 - ctx.cursor.x;
    let dy = y as i32 - ctx.cursor.y;
    post(ctx, INPUT_KIND_POINTER_REL, 0, 0, dx, dy);
}

pub fn down(ctx: &mut Context, button: u32) {
    post(ctx, INPUT_KIND_BUTTON_DOWN, button, 0, 0, 0);
}

pub fn up(ctx: &mut Context, button: u32) {
    post(ctx, INPUT_KIND_BUTTON_UP, button, 0, 0, 0);
}

pub fn wheel(ctx: &mut Context, step: i32) {
    post(ctx, INPUT_KIND_WHEEL, 0, 0, 0, step);
}

/// Everything delivered since the last call, in order.
pub fn sent() -> Vec<Rx> {
    nonos_libc::take_sent()
        .into_iter()
        .map(|(pid, f)| Rx {
            pid,
            kind: u16::from_le_bytes([f[8], f[9]]),
            flags: u16::from_le_bytes([f[10], f[11]]),
            code: u32::from_le_bytes([f[12], f[13], f[14], f[15]]),
            x: i32::from_le_bytes([f[16], f[17], f[18], f[19]]),
            y: i32::from_le_bytes([f[20], f[21], f[22], f[23]]),
            dy: i32::from_le_bytes([f[28], f[29], f[30], f[31]]),
        })
        .collect()
}

/// The deliveries of one kind, as (pid, code).
pub fn of_kind(rx: &[Rx], kind: u16) -> Vec<(u32, u32)> {
    rx.iter().filter(|r| r.kind == kind).map(|r| (r.pid, r.code)).collect()
}
