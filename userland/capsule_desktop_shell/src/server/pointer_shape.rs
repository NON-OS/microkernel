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
//! Whether a message may be taken as from one named peer. Any capsule can
//! Once a pass of the serve loop: the chrome's pixel under the pointer, and
//! the arrow asked for or withdrawn when that changed (state/ctx/
//! pointer_shape.rs). Once a pass, so a menu opened or closed under a
//! pointer at rest changes its shape too.

use crate::compositor_client::cursor_shape::cursor_shape;
use crate::state::Context;

pub fn sync(ctx: &mut Context) {
    let Some((x, y)) = ctx.pointer_shape.at else { return };
    let (handle, va) = (ctx.chrome.shown_handle(), ctx.chrome.shown_va());
    if handle == 0 || va == 0 || x >= ctx.width || y >= ctx.height {
        return;
    }
    let at = va + u64::from(y) * u64::from(ctx.stride) + u64::from(x) * 4;
    // SAFETY: (x, y) is on the chrome surface, which is mapped for the
    // shell's life, `stride` bytes a row.
    let pixel = unsafe { core::ptr::read_volatile(at as *const u32) };
    let Some(over) = ctx.pointer_shape.change(pixel) else { return };
    let rid = ctx.issue_request_id();
    if cursor_shape(ctx.ports.compositor_port, rid, handle, over).is_ok() {
        ctx.pointer_shape.said = Some(over);
    }
}
