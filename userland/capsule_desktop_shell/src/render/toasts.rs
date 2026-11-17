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

use alloc::vec::Vec;

use nonos_toolkit::widgets::{Rect, Toast};

use super::layout::bottom_dock_rect;
use super::surface::surface;
use super::toast_view::{bounds, draw, hit, stack};
use super::ui_font::{px, valid_str};
use crate::compositor_client::push_damage_commit;
use crate::state::{Context, TOAST_WINDOW_ID};
use crate::wm_client;

const TOAST_RIGHT_INSET_LOGICAL: u32 = 12;
const TOAST_DOCK_GAP_LOGICAL: u32 = 10;

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
    ctx.toast_layer_live = live;
    let was = ctx.toast_window.map(|(x, y, w, h)| Rect { x, y, w, h });
    let panel = bounds(&laid(ctx).1);
    claim_presses(ctx, panel);
    if let Some(r) = bounds(&[was, panel].into_iter().flatten().collect::<Vec<_>>()) {
        let rid = ctx.issue_request_id();
        let _ = push_damage_commit(ctx.compositor_port, rid, r.x, r.y, r.w, r.h);
    }
}

/// The toasts, last in the chrome's frame so they sit over everything in it.
pub fn paint_in_chrome(ctx: &mut Context) {
    ctx.toasts_drawn = ctx.toasts.generation();
    let (toasts, boxes) = laid(ctx);
    draw(&mut surface(ctx), &toasts, &boxes);
}

/// Each toast up, in its tone, and the box it is drawn in: stacked up from
/// above the dock at the right, the first up at the top.
fn laid(ctx: &Context) -> (Vec<Toast<'_>>, Vec<Rect>) {
    let toasts: Vec<Toast<'_>> = ctx
        .toasts
        .iter_live()
        .map(|t| Toast::new(valid_str(&t.text[..t.len])).tone(t.level.tone()))
        .collect();
    let inset = px(TOAST_RIGHT_INSET_LOGICAL);
    let foot = bottom_dock_rect(ctx.width, ctx.height).y.saturating_sub(px(TOAST_DOCK_GAP_LOGICAL));
    let room = ctx.width.saturating_sub(inset * 2);
    let boxes = stack(&toasts, ctx.width.saturating_sub(inset), foot, room);
    (toasts, boxes)
}

/// Whether a press at (`x`, `y`) lands on the toasts' panel.
pub fn toast_hit(ctx: &Context, x: u32, y: u32) -> bool {
    let (x, y) = (i32::try_from(x).unwrap_or(i32::MAX), i32::try_from(y).unwrap_or(i32::MAX));
    hit(&laid(ctx).1, x, y)
}

/// The panel is drawn in the chrome band, over every window; it is a window
/// of the shell's while it is up, so a press on it reaches the shell (which
/// dismisses it) instead of the window drawn under it. Reopened on every
/// change of its rect, since it is the bounds of the toasts' stack.
fn claim_presses(ctx: &mut Context, panel: Option<Rect>) {
    let want = panel.map(|p| (p.x, p.y, p.w, p.h));
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
            p.w,
            p.h,
        );
    }
}

const WINDOW_KIND_POPUP: u32 = 3;
