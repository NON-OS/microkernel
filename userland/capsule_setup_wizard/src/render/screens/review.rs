use crate::render;
use crate::render::theme::{FG, HINT};
use crate::render::widgets::{lines, text::cat};
use crate::server::step::{default_key, Outcome, K_ENTER, K_ENTER_LF};
use crate::state::Context;

use super::review_lines::{local_line, mode_line, name_line, net_line, route_line};
use super::{appearance, keyboard, timezone};
use crate::render::layout::REVIEW_VALUE_AT;

pub fn draw(ctx: &Context) {
    let then: &[u8] = if super::mode::keeps(ctx) {
        b"ENTER applies these and opens the installer"
    } else {
        b"ENTER applies these and starts the desktop"
    };
    render::frame(ctx, b"Review", then, b"ENTER FINISH  ESC BACK");
    let spx = ctx.stride as usize / 4;
    let (w, h) = (ctx.width, ctx.height);
    let l = render::layout_of(ctx);
    let (buf, x) = (render::buffer(ctx), l.col_x);
    let mut tz = [0u8; 6];
    let tz_len = timezone::label(ctx.tz_off, &mut tz);
    let net: &[u8] = ctx.net.joined.as_ref().map_or(b"None", |n| n.ssid());
    let (mut name, mut qwen, mut wall) = ([0u8; 64], [0u8; 64], [0u8; 64]);
    let empty: &[u8] = if ctx.name.len == 0 { b" (left empty)" } else { b"" };
    let name = cat(&mut name, &[ctx.name.shown(), empty]);
    let qwen: &[u8] = match ctx.qwen.chosen(super::mode::keeps(ctx)) {
        Some(tier) => cat(&mut qwen, &[crate::qwen::label(tier), b" (", tier, b")"]),
        None => b"None for now",
    };
    let kept = alloc::format!(", {} kept", appearance::count(ctx));
    let wall = cat(&mut wall, &[appearance::name(ctx), kept.as_bytes()]);
    let names: [&[u8]; 7] = [
        keyboard::label(ctx.kbd_sel),
        name,
        ctx.host.shown(),
        &tz[..tz_len],
        net,
        wall,
        qwen,
    ];
    let heads: [&[u8]; 7] =
        [b"Keyboard", b"Name", b"Computer", b"Time zone", b"Network", b"Wallpaper", b"Qwen model"];
    let mut y = l.body_y;
    for (head, name) in heads.iter().zip(names.iter()) {
        lines::text(buf, spx, w, h, x, y, &[head], HINT);
        y = lines::text(buf, spx, w, h, x + REVIEW_VALUE_AT * l.unit, y, &[name], FG);
    }
    let mut apps = [0u8; 96];
    let said = [
        mode_line(ctx),
        local_line(ctx),
        net_line(ctx),
        route_line(ctx),
        name_line(ctx),
        crate::apps::said(ctx, &mut apps),
    ];
    let y = lines::text(buf, spx, w, h, x, y + l.gap, &said, FG);
    let (mut first, mut rest) = ([0u8; 96], [0u8; 96]);
    let (a, b) = super::unsaved::said(ctx.unsaved, &mut first, &mut rest);
    if ctx.unsaved_told && a > 0 {
        let told: [&[u8]; 3] = [&first[..a], &rest[..b], super::unsaved::THEN];
        let shown: &[&[u8]] = if b == 0 { &[told[0], told[2]] } else { &told };
        lines::text(buf, spx, w, h, x, y + l.gap, shown, FG);
    }
}

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    if code == K_ENTER || code == K_ENTER_LF {
        /*
         * Answers the settings service refused are named before setup goes
         * on, once: the second Enter asks again and goes on either way.
         */
        super::commit::commit(ctx);
        if ctx.unsaved != 0 && !ctx.unsaved_told {
            ctx.unsaved_told = true;
            return Outcome::Stay;
        }
        return Outcome::Advance;
    }
    default_key(code)
}
