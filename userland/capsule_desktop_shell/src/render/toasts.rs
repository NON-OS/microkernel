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

use super::fill::fill_rect;
use super::layout::{bottom_dock_rect, Rect};
use super::measure_aa::{measure_aa_bytes, truncate_to_width};
use super::text_aa::text_aa;
use super::ui_font::{line_h, px, top_y_centered, valid_str, UI_PX};
use crate::compositor_client::push_damage_commit;
use crate::state::toasts::MAX_TOASTS;
use crate::state::{Context, TOAST_WINDOW_ID};
use crate::wm_client;

const TOAST_MAX_WIDTH_LOGICAL: u32 = 460;
const TOAST_MIN_WIDTH_LOGICAL: u32 = 260;
const TOAST_RIGHT_INSET_LOGICAL: u32 = 12;
const TOAST_DOCK_GAP_LOGICAL: u32 = 10;
const ROW_PAD_LOGICAL: u32 = 12;
const ROW_GAP_LOGICAL: u32 = 2;
const ACCENT_WIDTH_LOGICAL: u32 = 4;
const TEXT_INSET_LOGICAL: u32 = 12;
const PANEL_ARGB: u32 = 0xFF0E_1218;
const ROW_ARGB: u32 = 0xFF1B_2030;
const TEXT_ARGB: u32 = 0xFFCF_E6E9;

fn toast_max_width() -> u32 {
    px(TOAST_MAX_WIDTH_LOGICAL)
}

fn toast_min_width() -> u32 {
    px(TOAST_MIN_WIDTH_LOGICAL)
}

fn toast_right_inset() -> u32 {
    px(TOAST_RIGHT_INSET_LOGICAL)
}

fn toast_dock_gap() -> u32 {
    px(TOAST_DOCK_GAP_LOGICAL)
}

fn row_pad() -> u32 {
    px(ROW_PAD_LOGICAL)
}

fn row_gap() -> u32 {
    px(ROW_GAP_LOGICAL)
}

fn accent_width() -> u32 {
    px(ACCENT_WIDTH_LOGICAL)
}

fn text_inset() -> u32 {
    px(TEXT_INSET_LOGICAL)
}

fn panel_edge_inset() -> u32 {
    px(2)
}

fn row_height() -> u32 {
    line_h(UI_PX) + row_pad()
}

fn panel_height() -> u32 {
    row_gap() + MAX_TOASTS as u32 * (row_height() + row_gap())
}

fn row_chrome_width() -> u32 {
    panel_edge_inset() * 2 + accent_width() + text_inset() * 2
}

pub fn toast_rect(display_width: u32, display_height: u32) -> Rect {
    let dock = bottom_dock_rect(display_width, display_height);
    let height = panel_height();
    let width = toast_max_width().min(display_width.saturating_sub(toast_right_inset() * 2));
    let x = display_width.saturating_sub(width + toast_right_inset());
    let y = dock.y.saturating_sub(height + toast_dock_gap());
    Rect { x, y, width, height }
}

/// Bring the toasts' panel on screen in line with the queue: repaint the
/// chrome when the toasts changed since it was last painted, claim the
/// panel's presses, and show its rectangle.
///
/// The panel used to be painted straight onto the chrome surface here,
/// after a transparent fill of its whole rectangle. That fill cut a hole in
/// whatever the chrome held there (the Launchpad, a menu, a dialog), and
/// every chrome paint in between, on each hover or icon drag step, cleared
/// the panel without drawing it again, so the toasts vanished until the
/// next clock tick. The panel is part of the chrome's frame now
/// (`paint_in_chrome`), drawn over everything else in it.
pub fn sync_toast_layer(ctx: &mut Context) {
    ctx.toasts_synced = ctx.toasts.generation();
    let live = !ctx.toasts.is_empty();
    if !live && !ctx.toast_layer_live {
        return;
    }
    if ctx.toasts_drawn != ctx.toasts.generation() {
        super::paint_chrome(ctx);
    }
    let r = toast_rect(ctx.width, ctx.height);
    ctx.toast_layer_live = live;
    let panel = live.then(|| panel_rect(ctx, r));
    claim_presses(ctx, panel);
    let rid = ctx.issue_request_id();
    let _ = push_damage_commit(ctx.compositor_port, rid, r.x, r.y, r.width, r.height);
}

/// The toasts, last in the chrome's frame so they sit over everything in it.
pub fn paint_in_chrome(ctx: &mut Context) {
    ctx.toasts_drawn = ctx.toasts.generation();
    if !ctx.toasts.is_empty() {
        paint_toasts(ctx, toast_rect(ctx.width, ctx.height));
    }
}

/// Where the panel sits inside `r`: right-aligned, as wide as its widest toast.
fn panel_rect(ctx: &Context, r: Rect) -> Rect {
    let w = panel_width(ctx, r.width);
    Rect { x: r.x + (r.width - w), y: r.y, width: w, height: r.height }
}

fn panel_width(ctx: &Context, layer_width: u32) -> u32 {
    let widest = ctx
        .toasts
        .iter_live()
        .map(|t| measure_aa_bytes(&t.text[..t.len], UI_PX))
        .max()
        .unwrap_or(0);
    (widest + row_chrome_width()).clamp(toast_min_width().min(layer_width), layer_width)
}

/// Whether a press at (`x`, `y`) lands on the toasts' panel.
pub fn toast_hit(ctx: &Context, x: u32, y: u32) -> bool {
    if ctx.toasts.is_empty() {
        return false;
    }
    let r = toast_rect(ctx.width, ctx.height);
    let w = panel_width(ctx, r.width);
    let px = r.x + (r.width - w);
    x >= px && x < px + w && y >= r.y && y < r.y + r.height
}

/// The panel is drawn in the chrome band, over every window; it is a window
/// of the shell's while it is up, so a press on it reaches the shell (which
/// dismisses it) instead of the window drawn under it. Reopened on every
/// change of its rect, since its width follows the widest toast.
fn claim_presses(ctx: &mut Context, panel: Option<Rect>) {
    let want = panel.map(|p| (p.x, p.y, p.width, p.height));
    if want == ctx.toast_window {
        return;
    }
    ctx.toast_window = want;
    let rid = ctx.issue_request_id();
    let _ = wm_client::window_close(ctx.wm_port, rid, TOAST_WINDOW_ID);
    if let Some(p) = panel {
        let rid = ctx.issue_request_id();
        let _ = wm_client::window_open(
            ctx.wm_port,
            rid,
            TOAST_WINDOW_ID,
            WINDOW_KIND_POPUP,
            p.x,
            p.y,
            p.width,
            p.height,
        );
    }
}

const WINDOW_KIND_POPUP: u32 = 3;

fn paint_toasts(ctx: &Context, r: Rect) {
    let (va, st, w, h) = (ctx.backing_va, ctx.stride, ctx.width, ctx.height);
    let panel = panel_rect(ctx, r);
    let (panel_x, panel_w) = (panel.x, panel.width);
    let row_h = row_height();
    fill_rect(va, st, w, h, panel_x, r.y, panel_w, r.height, PANEL_ARGB);
    for (i, toast) in ctx.toasts.iter_live().enumerate() {
        let row_y = r.y + row_gap() + i as u32 * (row_h + row_gap());
        let inset = panel_edge_inset();
        let row_w = panel_w.saturating_sub(inset * 2);
        fill_rect(va, st, w, h, panel_x + inset, row_y, row_w, row_h, ROW_ARGB);
        fill_rect(va, st, w, h, panel_x + inset, row_y, accent_width(), row_h, toast.level.tint());
        let text_x = panel_x + inset + accent_width() + text_inset();
        let text_max_w = (panel_x + panel_w).saturating_sub(text_x + text_inset());
        let label = truncate_to_width(valid_str(&toast.text[..toast.len]), UI_PX, text_max_w);
        text_aa(ctx, text_x, top_y_centered(row_y, row_h, UI_PX), label, TEXT_ARGB, UI_PX);
    }
}
