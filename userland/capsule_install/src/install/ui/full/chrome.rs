/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * Setup's left panel: the gradient, the accent edge, the wordmark, and the
 * steps with setup's own marks, + done, > here, . still to come. Setup
 * shows as done only when it kept answers for the install to carry.
 */

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::font::render::{draw_text, draw_text_scaled};

use super::palette::{ACCENT, DOT_CUR, DOT_DONE, DOT_TODO, GRAD_BOT, GRAD_TOP, HINT};
use crate::install::full::setup_kept;
use crate::install::state::Screen;

const WORDMARK: &[u8] = b"N\xD8NOS";
const STEPS: [&[u8]; 5] =
    [b"This computer", b"Choose the disk", b"Erase plan", b"Write and read back", b"Installed"];

pub(super) fn panel_w(w: u32) -> u32 {
    w * 38 / 100
}

pub(super) fn panel(fb: &mut PaintBuffer, screen: Screen) {
    let (w, h, pw) = (fb.width, fb.height, panel_w(fb.width));
    for row in 0..h {
        fb.fill_rect(0, row, pw, 1, lerp(GRAD_TOP, GRAD_BOT, row, h));
    }
    fb.fill_rect(pw, 0, 2, h, ACCENT);
    let spx = fb.stride_words as usize;
    draw_text_scaled(fb.pixels, spx, w, h, 32, 40, WORDMARK, ACCENT, 3);
    draw_text(fb.pixels, spx, w, h, 34, 96, b"INSTALL TO THIS COMPUTER", HINT);
    let setup: (&[u8], &[u8], u32) = match setup_kept() {
        true => (b"+", b"First-boot setup", DOT_DONE),
        false => (b"-", b"First-boot setup: no answers kept", DOT_TODO),
    };
    let mut rows = [setup; 6];
    let now = screen.step() as usize;
    for (i, label) in STEPS.iter().enumerate() {
        rows[i + 1] = match (i + 1).cmp(&now) {
            core::cmp::Ordering::Less => (b"+", label, DOT_DONE),
            core::cmp::Ordering::Equal => (b">", label, DOT_CUR),
            core::cmp::Ordering::Greater => (b".", label, DOT_TODO),
        };
    }
    for (i, (mark, label, colour)) in rows.iter().enumerate() {
        let y = 150 + 22 * i as u32;
        draw_text(fb.pixels, spx, w, h, 34, y, mark, *colour);
        draw_text(fb.pixels, spx, w, h, 52, y, label, *colour);
    }
}

fn lerp(a: u32, b: u32, num: u32, den: u32) -> u32 {
    let chan = |sh: u32| -> u32 {
        let (ca, cb) = (((a >> sh) & 0xFF) as i32, ((b >> sh) & 0xFF) as i32);
        (ca + (cb - ca) * num as i32 / den.max(1) as i32) as u32
    };
    0xFF00_0000 | (chan(16) << 16) | (chan(8) << 8) | chan(0)
}
