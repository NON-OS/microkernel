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

//! NIRS op 0x0006, from the compositor: its display is now width by height,
//! two u32s, sent on every change. The cursor's edges move at once, not at
//! the next DISPLAY_INFO recheck. One-way and never answered, as op
//! 0x0005 is (trusted_hold.rs). A notice from any other process, of another
//! length, or with a zero side is dropped.

use nonos_libc::mk_uptime_ms;

use crate::clients::wire::lookup_pid;
use crate::protocol::read_u32;
use crate::route::trusted_hold::accept;
use crate::state::Context;

const COMPOSITOR: &[u8] = b"compositor";

pub fn handle(ctx: &mut Context, sender_pid: u32, body: &[u8]) {
    if body.len() != 8 || !accept(sender_pid, lookup_pid(COMPOSITOR)) {
        return;
    }
    let (Some(width), Some(height)) = (read_u32(body, 0), read_u32(body, 4)) else {
        return;
    };
    if ctx.cursor.adopt(width, height) {
        ctx.display_checked_ms = mk_uptime_ms();
        // The cursor kept its place in proportion: draw it where it now is.
        (ctx.cursor_x, ctx.cursor_y) = (ctx.cursor.x as u32, ctx.cursor.y as u32);
        ctx.cursor_dirty = true;
    }
}
