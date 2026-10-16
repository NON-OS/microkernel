//! The review screen: every answer, and what was not saved.

use super::super::review_lines::{local_line, mode_line, name_line, net_line, route_line};
use super::super::{appearance, keyboard, timezone};
use crate::render;
use crate::render::layout::REVIEW_VALUE_AT;
use crate::render::theme::{FG, HINT};
use crate::render::widgets::{lines, text::cat};
use crate::state::Context;

pub fn draw(ctx: &Context) {
    let then: &[u8] = if super::super::mode::keeps(ctx) {
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
    let qwen: &[u8] = match ctx.qwen.chosen(super::super::mode::keeps(ctx)) {
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
    let (a, b) = super::super::unsaved::said(ctx.unsaved, &mut first, &mut rest);
    if ctx.unsaved_told && a > 0 {
        let told: [&[u8]; 3] = [&first[..a], &rest[..b], super::super::unsaved::THEN];
        let shown: &[&[u8]] = if b == 0 { &[told[0], told[2]] } else { &told };
        lines::text(buf, spx, w, h, x, y + l.gap, shown, FG);
    }
}
