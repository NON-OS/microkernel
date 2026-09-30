/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * The Apps step: which optional apps this machine runs. Each is on until
 * the person turns it off; what the system cannot run without is listed as
 * required and has no switch.
 */

use nonos_policy_proto::apps::REQUIRED;
use nonos_toolkit::font::render::draw_text;

use crate::render::theme::{FG, HINT};
use crate::render::{self, widgets::lines, widgets::rows, widgets::text::cat};
use crate::server::step::{default_key, list_nav, Outcome};
use crate::state::Context;

const K_SPACE: u32 = 0x20;
const ROWS_Y: u32 = 110;

const OFF_MEANS: &[&[u8]] = &[
    b"Off: the app is not started, and its dock icon does not open it.",
    b"Linux and Qwen off: qwen and Linux packages do not run.",
    b"Drivers and the network start before setup; none is chosen here.",
];

pub fn draw(ctx: &Context) {
    let sub = b"j/k to move, SPACE turns the app on or off";
    render::frame(ctx, b"Apps", sub, b"SPACE ON/OFF  ENTER NEXT  ESC BACK");
    let spx = ctx.stride as usize / 4;
    let (w, h) = (ctx.width, ctx.height);
    let (buf, x) = (render::buffer(ctx), render::content_x(w));
    let (mut text, mut items) = ([[0u8; 32]; 8], [&b""[..]; 8]);
    let mut n = 0;
    for (app, out) in crate::apps::listed(ctx.apps_present).zip(text.iter_mut()) {
        let mark: &[u8] = if ctx.apps_off & app.bit != 0 { b"off  " } else { b"on   " };
        items[n] = cat(out, &[mark, app.name]);
        draw_text(buf, spx, w, h, x + 376, ROWS_Y + 30 * n as u32 + 8, app.purpose, HINT);
        n += 1;
    }
    rows::list(buf, spx, w, h, x, ROWS_Y, &items[..n], ctx.apps_sel as usize);
    let mut y = ROWS_Y + 30 * n as u32 + 16;
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
    y = lines::text(buf, spx, w, h, x + 16, y, &REQUIRED, HINT);
    lines::text(buf, spx, w, h, x, y + 12, OFF_MEANS, HINT);
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
