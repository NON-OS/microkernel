use nonos_app_skeleton::{KEY_DOWN, KEY_UP};

use crate::render::{self, widgets::rows};
use crate::server::step::{default_key, Outcome};
use crate::state::Context;

/// The range the policy store accepts.
const WEST: i8 = -12;
const EAST: i8 = 14;

/// "UTC", or "UTC+5" / "UTC-11", into `out`; returns the length.
pub fn label(off: i8, out: &mut [u8; 6]) -> usize {
    out[..3].copy_from_slice(b"UTC");
    if off == 0 {
        return 3;
    }
    out[3] = if off < 0 { b'-' } else { b'+' };
    let n = off.unsigned_abs();
    if n >= 10 {
        out[4] = b'0' + n / 10;
        out[5] = b'0' + n % 10;
        return 6;
    }
    out[4] = b'0' + n;
    5
}

pub fn draw(ctx: &Context) {
    render::frame(
        ctx,
        b"Time zone",
        b"Hours from UTC, used by the menu bar clock. j adds an hour, k takes one off.",
        b"ENTER NEXT  ESC BACK",
    );
    let spx = ctx.stride as usize / 4;
    let (w, h) = (ctx.width, ctx.height);
    let buf = render::buffer(ctx);
    let mut text = [0u8; 6];
    let n = label(ctx.tz_off, &mut text);
    let l = render::layout_of(ctx);
    rows::list(buf, spx, w, h, l.col_x, l.body_y, &[&text[..n]], 0);
}

pub fn on_key(ctx: &mut Context, code: u32) -> Outcome {
    match code {
        KEY_UP | 0x6B => ctx.tz_off = (ctx.tz_off - 1).max(WEST),
        KEY_DOWN | 0x6A => ctx.tz_off = (ctx.tz_off + 1).min(EAST),
        _ => return default_key(code),
    }
    Outcome::Stay
}
