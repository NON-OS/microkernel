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

//! The pointer handed back to compositing after a CURSOR was refused: the
//! plane hidden on every output that still answers, the primary marked as
//! having no cursor plane until it is replaced, and the arrow's box damaged
//! so the next frame composites it.

use crate::display::cursor_rule::message;
use crate::display::ops;
use crate::state::damage::Rect;
use crate::state::Context;

/// The composited arrow's box on the canvas, as the cursor handler damages.
const BOX: u32 = 32;

pub fn give_back(ctx: &mut Context) {
    hide(ctx);
    if let Some(w) = ctx.wire.as_mut() {
        w.cursor_offset = 0;
    }
    let c = ctx.cursor.current();
    let (w, h) = (BOX.min(ctx.width.saturating_sub(c.x)), BOX.min(ctx.height.saturating_sub(c.y)));
    if c.visible && w > 0 && h > 0 {
        ctx.damage.accumulate(Rect { x: c.x, y: c.y, width: w, height: h });
    }
}

/// The plane hidden on every output, the plane kept: the magnifier draws
/// the arrow into the canvas, and the next sync after it shows it again.
pub fn hide(ctx: &mut Context) {
    let off = message((0, 0), false, false, (0, 0), (0, 0));
    let _ = ops::cursor(ctx.gfx_port, 0, &off);
    for h in ctx.heads.iter().filter(|h| h.shaped) {
        let _ = ops::cursor(ctx.gfx_port, h.output, &off);
    }
    ctx.cursor_sent = None;
}
