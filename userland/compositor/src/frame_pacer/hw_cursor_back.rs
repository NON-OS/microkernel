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
//! having no cursor plane until it is replaced, and the box the pointer's
//! shape may cover damaged so the next frame composites it whole.

use crate::display::cursor_rule::message;
use crate::display::ops;
use crate::state::cursor_pick::reach;
use crate::state::Context;

pub fn give_back(ctx: &mut Context) {
    hide(ctx);
    if let Some(w) = ctx.wire.as_mut() {
        w.cursor_offset = 0;
    }
    // The box every composited shape fits in, from HOT_REACH up and left
    // of the hotspot, as a move or a shape change damages it: a resize
    // cursor is drawn round its hotspot, not from it.
    let c = ctx.cursor.current();
    let r = reach(c.x, c.y, ctx.width, ctx.height);
    if c.visible && r.width > 0 && r.height > 0 {
        ctx.damage.accumulate(r);
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
