/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * The computer-name step on screen: the name as typed, the rules, and why
 * the last key or Enter was refused.
 */

use nonos_policy_proto::setup_record::HOST_MAX;

use crate::host::{check, Refused};
use crate::render;
use crate::render::theme::{FG, HINT};
use crate::render::widgets::{lines, rows, text::cat};
use crate::server::step::Outcome;
use crate::state::Context;
use crate::text::{key, Typed};

const RULES: &[&[u8]] = &[
    b"Lowercase letters, digits and -, starting with a letter",
    b"and ending with a letter or a digit. Up to 63 characters.",
    b"Left empty, the computer is called nonos.",
    b"Install mode keeps it with the other answers; amnesic does not.",
];

pub fn draw(ctx: &Context) {
    render::frame(
        ctx,
        b"Computer name",
        b"What this machine calls itself: the host in name@host.",
        b"TYPE A NAME  ENTER NEXT  ESC BACK",
    );
    let spx = ctx.stride as usize / 4;
    let (w, h) = (ctx.width, ctx.height);
    let l = render::layout_of(ctx);
    let (buf, x) = (render::buffer(ctx), l.col_x);
    let mut shown = [0u8; HOST_MAX + 1];
    let typed = cat(&mut shown, &[ctx.host.typed(), b"_"]);
    let y = rows::list(buf, spx, w, h, x, l.body_y, &[typed], 0);
    let y = lines::text(buf, spx, w, h, x, y + l.gap, RULES, HINT);
    let mut line = [0u8; 80];
    let said = match ctx.host.refused {
        Some((_, Refused::DashLast)) => cat(&mut line, &[b"Not yet: ", Refused::DashLast.text()]),
        Some((byte, why)) => cat(&mut line, &[b"Refused \"", &[byte], b"\": ", why.text()]),
        None => return,
    };
    lines::text(buf, spx, w, h, x, y + l.gap, &[said], FG);
}

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    let n = &mut ctx.host;
    match key(&mut n.line, &mut n.len, code, check) {
        Typed::Changed => n.refused = None,
        Typed::Refused(why) => n.refused = Some((code as u8, why)),
        Typed::Enter if n.typed().last() == Some(&b'-') => {
            n.refused = Some((b'-', Refused::DashLast));
        }
        Typed::Enter => return Outcome::Advance,
        Typed::Esc => return Outcome::Back,
        Typed::Ignored => {}
    }
    Outcome::Stay
}
