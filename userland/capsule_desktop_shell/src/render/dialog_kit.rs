// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! What every shell dialog is drawn with, so they all look alike: its
//! colours, its margin, a filled and bordered panel, and a button whose
//! keyboard focus wears a ring in the accent.

use crate::render::fill::fill_rect;
use crate::render::layout::Rect;
use crate::render::measure_aa::{measure_aa, truncate_to_width};
use crate::render::palette;
use crate::render::text_aa::text_aa;
use crate::render::ui_font;
use crate::render::ui_font::{top_y_centered, UI_PX};
use crate::state::Context;

pub(super) const PANEL: u32 = 0xFF12_1A26;
pub(super) const BORDER: u32 = 0xFF2E_3A4C;
pub(super) const APPROVE_BG: u32 = 0xFF1E_7A3C;
pub(super) const CANCEL_BG: u32 = 0xFF3A_2430;
pub(super) const FG: u32 = 0xFFDF_EAF7;
pub(super) const DIM: u32 = 0xFFAF_BED2;
const MARGIN_LOGICAL: u32 = 20;

pub(super) fn margin() -> u32 {
    ui_font::px(MARGIN_LOGICAL)
}

/// A dialog button; the one the keyboard is on (state::dialog_keys) wears
/// a ring in the accent, so Enter's target is never a guess.
pub(super) fn button(ctx: &Context, r: Rect, bg: u32, label: &str, focused: bool) {
    fill(ctx, r, bg);
    border(ctx, r, BORDER);
    if focused {
        let s = ui_font::scale();
        border(ctx, r, palette::ACCENT);
        let inner = Rect {
            x: r.x + s,
            y: r.y + s,
            width: r.width.saturating_sub(2 * s),
            height: r.height.saturating_sub(2 * s),
        };
        border(ctx, inner, palette::ACCENT);
    }
    let inner = r.width.saturating_sub(ui_font::px(8));
    let fitted = truncate_to_width(label, UI_PX, inner);
    let tx = r.x + r.width.saturating_sub(measure_aa(fitted, UI_PX)) / 2;
    text_aa(ctx, tx, top_y_centered(r.y, r.height, UI_PX), fitted, FG, UI_PX);
}

pub(super) fn fill(ctx: &Context, r: Rect, argb: u32) {
    fill_rect(ctx.backing_va, ctx.stride, ctx.width, ctx.height, r.x, r.y, r.width, r.height, argb);
}

pub(super) fn border(ctx: &Context, r: Rect, argb: u32) {
    let (va, st, w, h) = (ctx.backing_va, ctx.stride, ctx.width, ctx.height);
    let s = ui_font::scale();
    fill_rect(va, st, w, h, r.x, r.y, r.width, s, argb);
    fill_rect(va, st, w, h, r.x, r.y + r.height.saturating_sub(s), r.width, s, argb);
    fill_rect(va, st, w, h, r.x, r.y, s, r.height, argb);
    fill_rect(va, st, w, h, r.x + r.width.saturating_sub(s), r.y, s, r.height, argb);
}
