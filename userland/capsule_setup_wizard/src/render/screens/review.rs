use crate::render;
use crate::render::theme::FG;
use crate::render::widgets::{lines, text::cat};
use crate::server::step::{default_key, Outcome, K_ENTER, K_ENTER_LF};
use crate::state::Context;

use super::review_lines::{local_line, mode_line, name_line, net_line};
use super::{appearance, keyboard, timezone};

pub fn draw(ctx: &Context) {
    let then: &[u8] = if super::mode::keeps(ctx) {
        b"ENTER applies these and opens the installer"
    } else {
        b"ENTER applies these and starts the desktop"
    };
    render::frame(ctx, b"Review", then, b"ENTER FINISH  ESC BACK");
    let spx = ctx.stride as usize / 4;
    let (w, h) = (ctx.width, ctx.height);
    let (buf, x) = (render::buffer(ctx), render::content_x(w));
    let mut tz = [0u8; 6];
    let tz_len = timezone::label(ctx.tz_off, &mut tz);
    let net: &[u8] = ctx.net.joined.as_ref().map_or(b"None", |n| n.ssid());
    let (mut name, mut qwen) = ([0u8; 64], [0u8; 64]);
    let empty: &[u8] = if ctx.name.len == 0 { b" (left empty)" } else { b"" };
    let name = cat(&mut name, &[ctx.name.shown(), empty]);
    let qwen: &[u8] = match ctx.qwen.chosen() {
        Some(tier) => cat(&mut qwen, &[crate::qwen::label(tier), b" (", tier, b")"]),
        None => b"None for now",
    };
    let names: [&[u8]; 6] = [
        keyboard::label(ctx.kbd_sel),
        name,
        &tz[..tz_len],
        net,
        appearance::name(ctx.wall_sel),
        qwen,
    ];
    let heads: [&[u8]; 6] =
        [b"Keyboard", b"Name", b"Time zone", b"Network", b"Wallpaper", b"Qwen model"];
    for (i, (head, name)) in heads.iter().zip(names.iter()).enumerate() {
        let y = 110 + 20 * i as u32;
        lines::text(buf, spx, w, h, x, y, &[head], FG);
        lines::text(buf, spx, w, h, x + 110, y, &[name], FG);
    }
    let mut apps = [0u8; 96];
    let said = [
        mode_line(ctx),
        local_line(ctx),
        net_line(ctx),
        name_line(ctx),
        crate::apps::said(ctx, &mut apps),
    ];
    lines::text(buf, spx, w, h, x, 250, &said, FG);
}

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    if code == K_ENTER || code == K_ENTER_LF {
        super::commit::commit(ctx);
        return Outcome::Advance;
    }
    default_key(code)
}
