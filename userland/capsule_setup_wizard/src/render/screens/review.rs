use crate::render::theme::FG;
use crate::render::{self, widgets::lines};
use crate::server::step::{default_key, Outcome, K_ENTER, K_ENTER_LF};
use crate::state::Context;

use super::{appearance, keyboard, mode, timezone};

fn mode_line(ctx: &Context) -> &'static [u8] {
    match (mode::keeps(ctx), crate::keep::store_ready()) {
        (false, _) => b"Mode: amnesic. Nothing is kept; setup runs again next boot.",
        (true, true) => b"Mode: install. Answers kept, then the installer opens.",
        (true, false) => b"Mode: install. No NONOS store this boot: nothing is kept.",
    }
}

fn net_line(ctx: &Context) -> &'static [u8] {
    match (ctx.net.joined.is_some(), ctx.net.remember && mode::keeps(ctx)) {
        (false, _) => b"Wi-Fi: none joined.",
        (true, true) => b"Wi-Fi: joined, remembered sealed with the TPM key.",
        (true, false) => b"Wi-Fi: joined for this boot only; nothing is kept.",
    }
}

fn local_line(ctx: &Context) -> &'static [u8] {
    /*
     * Named here too, since this commit is what grants or revokes it.
     */
    match (ctx.local_sel, mode::keeps(ctx)) {
        (1, true) => b"Installed software may run",
        (1, false) => b"Installed software may run, this boot",
        _ => b"Only NONOS software runs",
    }
}

pub fn draw(ctx: &Context) {
    render::frame(
        ctx,
        b"Review",
        b"ENTER applies these and starts the desktop",
        b"ENTER FINISH  ESC BACK",
    );
    let spx = ctx.stride as usize / 4;
    let (w, h) = (ctx.width, ctx.height);
    let (buf, x) = (render::buffer(ctx), render::content_x(w));
    let mut tz = [0u8; 6];
    let tz_len = timezone::label(ctx.tz_off, &mut tz);
    let net: &[u8] = ctx.net.joined.as_ref().map_or(b"None", |n| n.ssid());
    let names: [&[u8]; 4] =
        [keyboard::label(ctx.kbd_sel), &tz[..tz_len], net, appearance::name(ctx.wall_sel)];
    let heads: [&[u8]; 4] = [b"Keyboard", b"Time zone", b"Network", b"Wallpaper"];
    for (i, (head, name)) in heads.iter().zip(names.iter()).enumerate() {
        let y = 110 + 20 * i as u32;
        lines::text(buf, spx, w, h, x, y, &[head], FG);
        lines::text(buf, spx, w, h, x + 100, y, &[name], FG);
    }
    lines::text(buf, spx, w, h, x, 210, &[mode_line(ctx), local_line(ctx), net_line(ctx)], FG);
}

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    if code == K_ENTER || code == K_ENTER_LF {
        super::commit::commit(ctx);
        return Outcome::Advance;
    }
    default_key(code)
}
