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
use crate::display::head_rule::{head_slot, head_step, heads_missing, heads_wanted, HeadStep};
use crate::display::{ops, Head};
use crate::state::Context;

pub fn poll_heads(ctx: &mut Context, port: u32) -> Option<&'static str> {
    let outputs = ops::caps(port).ok()?.outputs;
    let link = ctx.wire.as_mut()?;
    if outputs != link.outputs {
        link.outputs = outputs;
        let had = ctx.own_colours();
        release_heads(&mut ctx.heads);
        ctx.heads = prime_heads(port, outputs);
        shown_anew(ctx, had);
        return Some("outputs plugged or pulled; laid out again");
    }
    if ctx.heads.len() < heads_wanted(outputs) {
        prime_missing(ctx, port, outputs);
    }
    let mut what = None;
    for i in 0..ctx.heads.len() {
        let h = ctx.heads[i];
        match head_step(&h, ops::state(port, h.output).ok()) {
            HeadStep::Keep => continue,
            HeadStep::Repaint => {
                (ctx.heads[i].shaped, ctx.heads[i].owed) = (false, false);
                what = Some("a mirrored output lost and set again");
            }
            HeadStep::Resend => {
                ctx.heads[i].owed = false;
                what = Some("a mirrored output's refused frame sent again");
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

/// The heads the output count asks for that did not answer before, asked
/// again; those that answer join the others in output order and the whole
/// screen is sent. One that still does not answer changes nothing: the
/// heads already shown keep their primaries and nothing is repainted.
fn prime_missing(ctx: &mut Context, port: u32, outputs: u16) {
    let (have, n) = outputs_of(&ctx.heads);
    let had = ctx.own_colours();
    let mut added = false;
    for o in heads_missing(outputs, &have[..n]) {
        let Some(new) = head(port, o) else { continue };
        let (now, k) = outputs_of(&ctx.heads);
        ctx.heads.insert(head_slot(o, &now[..k]), new);
        added = true;
    }
    if added {
        shown_anew(ctx, had);
    }
}

/// The outputs the heads show, in their order.
fn outputs_of(heads: &[Head]) -> ([u16; MAX_HEADS], usize) {
    let mut have = [0u16; MAX_HEADS];
    let n = heads.len().min(MAX_HEADS);
    for (slot, h) in have.iter_mut().zip(heads) {
        *slot = h.output;
    }
    (have, n)
}

/// The heads changed: mirrors in their own colours need a canvas of their
/// own, or not, and every head is sent the whole screen.
fn shown_anew(ctx: &mut Context, had: bool) {
    if ctx.want_extend || had != ctx.own_colours() {
        super::layout::relayout(ctx);
    }
    ctx.damage.mark_full(ctx.width, ctx.height);
}

/// The mirrored outputs a wire server keeps past output 0.
const MAX_HEADS: usize = 3;
