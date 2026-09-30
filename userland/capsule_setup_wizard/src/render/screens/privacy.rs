use crate::render::theme::FG;
use crate::render::{self, widgets::lines};
use crate::server::step::{default_key, Outcome};
use crate::state::Context;

/*
 * Nothing here is a switch, because nothing reads one: each line says what
 * the code does on every boot, whatever setup is told.
 */
const FACTS: &[&[u8]] = &[
    b"Network addresses: the e1000, RTL8169 and RTL8821CE drivers",
    b"transmit from a random MAC drawn at each start, never the",
    b"factory one.",
    b"",
    b"Shutdown and reboot wipe process memory, kernel stacks and",
    b"held keys before the machine powers off or restarts.",
    b"",
    b"Telemetry: NONOS has none, so there is no switch for it.",
];

pub fn draw(ctx: &Context) {
    render::frame(ctx, b"Privacy", b"What NONOS does without being asked", b"ENTER NEXT  ESC BACK");
    let spx = ctx.stride as usize / 4;
    let (w, h) = (ctx.width, ctx.height);
    let buf = render::buffer(ctx);
    lines::text(buf, spx, w, h, render::content_x(w), 110, FACTS, FG);
}

pub fn on_key(_ctx: &mut Context, code: u32) -> Outcome {
    default_key(code)
}
