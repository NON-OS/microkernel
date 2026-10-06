// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

/*
 * Setup's left panel, as on the installer and the loader: the NØNOS emblem,
 * what this is, and the steps as numbered mono rows, the current one lit.
 * The layout centres the whole block top to bottom and picks its row height.
 */

use alloc::format;

use nonos_app_skeleton::PaintBuffer;
use nonos_brand::palette::{CYAN, RULE, TEXT, TEXT_3};
use nonos_brand::{emblem, label, label_w, line_h, release, Face};

use crate::render::layout::Layout;
use crate::render::theme::STEP_LABELS;
use crate::state::Context;

pub fn panel(buf: &mut [u32], spx: usize, l: &Layout, ctx: &Context) {
    let (w, h, pw, px) = (l.width, l.height, l.panel_w, l.label_px);
    let mut fb = PaintBuffer { pixels: buf, stride_words: spx as u32, width: w, height: h };
    fb.fill_rect(pw, 0, l.scale.px(1).max(1), h, RULE);
    emblem(&mut fb, l.emblem_x, l.emblem_y, l.emblem_w, CYAN, true);
    centred(&mut fb, pw, l.caption_y, "FIRST-BOOT SETUP", TEXT, px);
    centred(&mut fb, pw, l.release_y, &release(), TEXT_3, px);
    let (x, mut y) = (l.steps_x, l.steps_y);
    let dy = l.step_h.saturating_sub(line_h(Face::Mono, px)) / 2;
    let cur = ctx.step as usize;
    for (i, name) in STEP_LABELS.iter().enumerate() {
        let caps: alloc::string::String =
            name.iter().map(|&b| (b as char).to_ascii_uppercase()).collect();
        let colour = if i < cur {
            CYAN
        } else if i == cur {
            TEXT
        } else {
            TEXT_3
        };
        if i == cur {
            let bar = l.scale.px(2);
            let bar_h = l.step_h.saturating_sub(2 * dy).max(bar);
            fb.fill_round(x.saturating_sub(2 * l.unit) + bar, y + dy, bar, bar_h, 1, CYAN);
        }
        let n = label(
            &mut fb,
            x,
            y + dy,
            &format!("{:02}", i + 1),
            if i == cur { CYAN } else { TEXT_3 },
            px,
        );
        label(&mut fb, n + l.unit + l.unit / 4, y + dy, &caps, colour, px);
        y += l.step_h;
    }
}

fn centred(fb: &mut PaintBuffer, pw: u32, y: u32, s: &str, c: u32, px: f32) {
    label(fb, pw.saturating_sub(label_w(s, px)) / 2, y, s, c, px);
}
