//! Which network the machine's own traffic leaves through by default: the
//! browser starts on it and Qwen downloads take it. Each row says plainly
//! what it hides and what it costs; the mixnet is first and the default.

use nonos_policy_proto::route::{ANYONE, DIRECT, NYM};

use crate::render::theme::FG;
use crate::render::{self, widgets::lines, widgets::rows};
use crate::server::step::{default_key, list_nav, Outcome};
use crate::state::Context;

const ROUTES: &[&[u8]] = &[b"Nym mixnet (default)", b"Anyone network", b"Direct"];

const WHY: [&[&[u8]]; 3] = [
    &[
        b"Hides who you talk to, even from someone watching the whole",
        b"internet: every packet is mixed and delayed on its way.",
        b"Pages and downloads are slow; a large Qwen model can take hours.",
    ],
    &[
        b"Onion routing through three relays, as Tor does. Sites never see",
        b"this machine's address, and it is much faster than the mixnet.",
        b"Someone watching both ends at once could match the traffic.",
    ],
    &[
        b"No anonymity network: the fastest, and nothing hides you.",
        b"Every site, the model mirror and your own network see this",
        b"machine's address.",
    ],
];

const AFTER: &[&[u8]] = &[
    b"The browser can still switch network for a page, and Settings",
    b"changes this later. A route that fails never falls back to another.",
];

/// The route the row stands for, as nonos_policy_proto::route has it.
pub fn route(sel: u8) -> u8 {
    match sel {
        1 => ANYONE,
        2 => DIRECT,
        _ => NYM,
    }
}

pub fn draw(ctx: &Context) {
    render::frame(ctx, b"Network route", b"Which network this machine's own traffic leaves through",
        b"UP DOWN TO CHOOSE  ENTER NEXT  ESC BACK");
    let spx = ctx.stride as usize / 4;
    let (w, h) = (ctx.width, ctx.height);
    let l = render::layout_of(ctx);
    let (buf, x) = (render::buffer(ctx), l.col_x);
    let y = rows::list(buf, spx, w, h, x, l.body_y, ROUTES, ctx.route_sel as usize);
    let y = lines::text(buf, spx, w, h, x, y + l.gap, WHY[ctx.route_sel as usize % WHY.len()], FG);
    lines::text(buf, spx, w, h, x, y + l.line_h, AFTER, FG);
}

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    if let Some(o) = list_nav(&mut ctx.route_sel, ROUTES.len() as u8, code) {
        return o;
    }
    default_key(code)
}
