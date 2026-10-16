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

use crate::state::Context;

use super::show::show;
use super::to_screen::to_screen;

pub fn tick(ctx: &mut Context) -> Result<(), &'static str> {
    // A dark lid presents nothing; its damage waits for the wake, which
    // damages the whole canvas anyway (state/lid_rule.rs).
    if super::first_frame::holding(ctx) || !ctx.lid.presents() {
        return Ok(());
    }
    let done = frame(ctx);
    ctx.lid = ctx.lid.after_frame(done.is_err());
    done
}

fn frame(ctx: &mut Context) -> Result<(), &'static str> {
    super::band_watch::watch(ctx);
    super::lens::refresh(ctx);
    // Composite every damaged rectangle this frame. Each is small and separate,
    // so the empty space between far-apart damage is never touched. A
    // rectangle the display refused stays damaged for the next frame. On the
    // wire a frame goes in batches, one FLUSH each.
    if ctx.wire.is_some() {
        super::hw_cursor::sync(ctx);
        let prepare = |ctx: &mut Context, r| {
            let shown = to_screen(ctx, r);
            super::extend::draw_heads(ctx, r);
            shown.and_then(|r| super::clip::clip_to_screen(ctx, r))
        };
        let send = super::present_wire::present_rects;
        let done = super::drain_batch::drain_batched(ctx, |ctx| &mut ctx.damage, prepare, send);
        super::extend::flush_heads(ctx);
        return done;
    }
    super::drain_damage::drain_damage(ctx, |ctx| &mut ctx.damage, show)
}
