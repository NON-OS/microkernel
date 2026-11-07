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

//! The composited pointer's shape, followed once a frame: a window that
//! moved, closed or went to the dock under a still pointer changes the shape
//! there with no motion and no OP_CURSOR_SHAPE, so its box is repainted then
//! (state/cursor_pick.rs reshaped), as the cursor plane is redrawn
//! (hw_cursor.rs).

use crate::state::cursor_pick::{reshaped, shape_at};
use crate::state::Context;

pub fn follow(ctx: &mut Context) {
    let c = ctx.cursor.current();
    if ctx.hw_cursor() || !c.visible {
        return;
    }
    let (layers, n) = ctx.scene.z_sorted_snapshot();
    let want = shape_at(layers[..n].iter(), c.x, c.y);
    let size = (ctx.width, ctx.height);
    if let Some(r) = reshaped(c, false, (ctx.sw_cursor_code, want), size) {
        ctx.damage.accumulate(r);
    }
    ctx.sw_cursor_code = want;
}
