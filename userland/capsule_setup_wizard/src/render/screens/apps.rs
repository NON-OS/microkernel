/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * The Apps step: which optional apps this machine runs. Each is on until
 * the person turns it off; what the system cannot run without is listed as
 * required and has no switch.
 */

use crate::render::ink::{body_line, draw_text};
use crate::render::layout::APPS_PURPOSE_AT;
use nonos_policy_proto::apps::REQUIRED;

use crate::render::theme::{FG, HINT};
use crate::render::{self, widgets::lines, widgets::rows, widgets::text::cat};
use crate::server::step::{default_key, list_nav, Outcome};
use crate::state::Context;

const K_SPACE: u32 = 0x20;

const OFF_MEANS: &[&[u8]] = &[
    b"Off: the app is not started, and its dock icon does not open it.",
    b"Linux and Qwen off: qwen and Linux packages do not run.",
    b"Drivers and the network start before setup; none is chosen here.",
];

pub fn draw(ctx: &Context) {
    let sub = b"Up and Down to move, SPACE turns the app on or off";
    render::frame(ctx, b"Apps", sub, b"SPACE ON/OFF  ENTER NEXT  ESC BACK");
    let spx = ctx.stride as usize / 4;
    let (w, h) = (ctx.width, ctx.height);
    let l = render::layout_of(ctx);
    let (buf, x) = (render::buffer(ctx), l.col_x);
    let (mut text, mut items) = ([[0u8; 32]; 8], [&b""[..]; 8]);
    let mut n = 0;
    for (app, out) in crate::apps::listed(ctx.apps_present).zip(text.iter_mut()) {
        let mark: &[u8] = if ctx.apps_off & app.bit != 0 { b"off  " } else { b"on   " };
        items[n] = cat(out, &[mark, app.name]);
        n += 1;
    }
    let mut y = rows::list(buf, spx, w, h, x, l.body_y, &items[..n], ctx.apps_sel as usize);
    // Each app's purpose in its own row, drawn after the rows so the lit row's
    // fill is under it rather than over it.
    let text_dy = l.row_h.saturating_sub(body_line(w, h)) / 2;
    for (i, app) in crate::apps::listed(ctx.apps_present).take(n).enumerate() {
        let top = l.body_y + l.row_h * i as u32 + text_dy;
        draw_text(buf, spx, w, h, x + APPS_PURPOSE_AT * l.unit, top, app.purpose, HINT);
    }
    y += l.gap;
    if n == 0 {
        y = lines::text(
            buf,
            spx,
            w,
            h,
            x,
            y,
            &[b"This image carries no app that can be turned off."],
            FG,
        );
    }
    y = lines::text(buf, spx, w, h, x, y, &[b"Required, always on:"], FG);
    y = lines::text(buf, spx, w, h, x + 2 * l.unit, y, &REQUIRED, HINT);
    lines::text(buf, spx, w, h, x, y + l.gap, OFF_MEANS, HINT);
}

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    let listed = crate::apps::listed(ctx.apps_present).count() as u8;
    if let Some(o) = list_nav(&mut ctx.apps_sel, listed, code) {
        return o;
    }
    if code != K_SPACE {
        return default_key(code);
    }
    if let Some(app) = crate::apps::listed(ctx.apps_present).nth(ctx.apps_sel as usize) {
        ctx.apps_off ^= app.bit;
    }
    Outcome::Stay
}
