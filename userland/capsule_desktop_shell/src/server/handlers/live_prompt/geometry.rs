// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Where the live session prompt and its two buttons sit, shared by the click
//! routing and the paint pass so the two never disagree.

use crate::render::layout::Rect;
use crate::render::ui_font;

const PANEL_W_LOGICAL: u32 = 520;
const PANEL_H_LOGICAL: u32 = 200;
const BTN_W_LOGICAL: u32 = 150;
const BTN_H_LOGICAL: u32 = 34;

pub(crate) fn panel_rect(w: u32, h: u32) -> Rect {
    let pw = ui_font::px(PANEL_W_LOGICAL).min(w);
    let ph = ui_font::px(PANEL_H_LOGICAL).min(h);
    Rect { x: w.saturating_sub(pw) / 2, y: h.saturating_sub(ph) / 2, width: pw, height: ph }
}

pub(crate) fn install_rect(w: u32, h: u32) -> Rect {
    button_rect(w, h, true)
}

pub(crate) fn later_rect(w: u32, h: u32) -> Rect {
    button_rect(w, h, false)
}

fn button_rect(w: u32, h: u32, install: bool) -> Rect {
    let p = panel_rect(w, h);
    let (bw, bh) = (ui_font::px(BTN_W_LOGICAL), ui_font::px(BTN_H_LOGICAL));
    let y = p.y + p.height.saturating_sub(bh + ui_font::px(18));
    let mid = p.x + p.width / 2;
    let x = if install { mid.saturating_sub(bw + ui_font::px(8)) } else { mid + ui_font::px(8) };
    Rect { x, y, width: bw, height: bh }
}

pub(super) fn hit(r: Rect, px: u32, py: u32) -> bool {
    px >= r.x && px < r.x + r.width && py >= r.y && py < r.y + r.height
}
