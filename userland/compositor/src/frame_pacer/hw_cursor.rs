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

//! The pointer on the wire driver's cursor plane (CURSOR, section 12): the
//! shape drawn into a primary's slot the first time it is shown there and
//! announced with NEW_SHAPE, then the position once a frame to every output
//! when it moved, in place of compositing it and damaging around it. A
//! refusal on any output hands the pointer back to compositing on all of
//! them, so no output shows two arrows or none.

use super::hw_cursor_send::send;
use crate::state::cursor_pick::shape_at;
use crate::state::Context;

pub fn sync(ctx: &mut Context) {
    // A window under the pointer that asks for another shape owes every
    // output the new shape, drawn and announced as the first one was.
    let c = ctx.cursor.current();
    let (layers, n) = ctx.scene.z_sorted_snapshot();
    let want = shape_at(layers[..n].iter(), c.x, c.y);
    if ctx.hw_cursor() && want != ctx.cursor_code {
        ctx.cursor_code = want;
        ctx.cursor_sent = None;
        ctx.heads.iter_mut().for_each(|h| h.shaped = false);
    }
    if !ctx.hw_cursor()
        || (ctx.cursor_sent == Some(ctx.lid.pointer(ctx.cursor.current())) && all_shaped(ctx))
    {
        return;
    }
    if send(ctx).is_none() {
        super::hw_cursor_back::give_back(ctx);
    }
}

fn all_shaped(ctx: &Context) -> bool {
    ctx.heads.iter().all(|h| h.shaped)
}
