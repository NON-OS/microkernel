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

//! An event delivered to the process holding input: a grab's holder, or the
//! trusted band's (`preempt`). Delivery is redirected, but the cursor keeps
//! tracking: the compositor's pointer position only ever comes from this
//! routing step, and parking it while a drag grab is held freezes the cursor
//! on screen. The holder also expects screen pixels, not raw device units,
//! so a pointer event carries the routed position.

use nonos_libc::InputEvent;

use crate::state::Context;

use super::deliver::deliver_one;
use super::pointer;

/// Deliver `event` to `holder`: how many went to it (0 or 1), and how many
/// button releases went to the window a press was in.
pub fn deliver_held(ctx: &mut Context, holder: u32, event: &InputEvent) -> (u32, u32) {
    let mut ev = *event;
    if super::kinds::is_pointer(ev.kind) {
        let (x, y) = ctx.cursor.apply(&ev);
        ctx.cursor_x = x;
        ctx.cursor_y = y;
        ctx.cursor_dirty = true;
        ev.x = x as i32;
        ev.y = y as i32;
    }
    let n = deliver_one(holder, &ev);
    super::recent::note(ctx, holder, ev.kind, n);
    let pressed = if super::kinds::is_pointer(ev.kind) {
        pointer::release_under_grab(ctx, holder, event, ev.x as u32, ev.y as u32)
    } else {
        0
    };
    (n, pressed)
}
