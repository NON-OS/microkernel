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

//! The mirrored outputs' part of the 500 ms poll: CAPS' output count (a
//! monitor plugged in or pulled out changes it, and the heads are primed
//! again), then each head's STATE and the step `head_rule` names.

use nonos_libc::mk_surface_release;

use super::heads::{head, prime_heads, release_heads};
use crate::display::head_rule::{head_step, heads_wanted, HeadStep};
use crate::display::ops;
use crate::state::Context;

pub fn poll_heads(ctx: &mut Context, port: u32) -> Option<&'static str> {
    let outputs = ops::caps(port).ok()?.outputs;
    let link = ctx.wire.as_mut()?;
    if outputs != link.outputs || ctx.heads.len() < heads_wanted(outputs) {
        let plugged = outputs != link.outputs;
        link.outputs = outputs;
        let had = ctx.own_colours();
        release_heads(&mut ctx.heads);
        ctx.heads = prime_heads(port, outputs);
        // Mirrors in their own colours need a canvas of their own, or not.
        if ctx.want_extend || had != ctx.own_colours() {
            super::layout::relayout(ctx);
        }
        ctx.damage.mark_full(ctx.width, ctx.height);
        return plugged.then_some("outputs plugged or pulled; laid out again");
    }
    let mut what = None;
    for i in 0..ctx.heads.len() {
        let h = ctx.heads[i];
        match head_step(&h, ops::state(port, h.output).ok()) {
            HeadStep::Keep => continue,
            HeadStep::Repaint => {
                ctx.heads[i].shaped = false;
                what = Some("a mirrored output lost and set again");
            }
            HeadStep::Resurface => {
                let Some(new) = head(port, h.output) else { continue };
                let _ = mk_surface_release(h.handle);
                ctx.heads[i] = new;
                if ctx.want_extend {
                    super::layout::relayout(ctx);
                }
                what = Some("a mirrored output's primary replaced");
            }
        }
        ctx.damage.mark_full(ctx.width, ctx.height);
    }
    what
}
