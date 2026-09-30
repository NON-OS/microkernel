/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * The review screen's line for the Apps step: the apps turned off by name,
 * or how many when their names would run past the screen.
 */

use crate::render::widgets::text::cat;
use crate::state::Context;

/* Characters the review column holds on the narrowest screen setup draws. */
const FITS: usize = 62;

pub fn said<'a>(ctx: &Context, out: &'a mut [u8; 96]) -> &'a [u8] {
    let off_bits = ctx.apps_present & ctx.apps_off;
    if ctx.apps_present == 0 {
        return b"Apps: this image carries none that can be turned off.";
    }
    if off_bits == 0 {
        return b"Apps: every app listed is on.";
    }
    let mut len = cat(out, &[b"Apps off:"]).len();
    for app in super::listed(off_bits) {
        let sep: &[u8] = if len > 9 { b", " } else { b" " };
        if len + sep.len() + app.name.len() > FITS {
            let (off, all) = (off_bits.count_ones() as u8, ctx.apps_present.count_ones() as u8);
            let (off, all) = ([b'0' + off], [b'0' + all]);
            return cat(
                out,
                &[b"Apps off: ", &off, b" of the ", &all, b" listed on the Apps step."],
            );
        }
        let (head, tail) = out.split_at_mut(len);
        len = head.len() + cat(tail, &[sep, app.name]).len();
    }
    &out[..len]
}
