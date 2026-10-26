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

//! The panel's presses: whether one lands on it, and the shell's own popup
//! window over its rectangle while it is up, so presses on it reach the
//! shell, which dismisses it.

use super::place::{panel_width, toast_rect};
use crate::render::layout::Rect;
use crate::state::{Context, TOAST_WINDOW_ID};
use crate::wm_client;

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
pub(super) fn claim_presses(ctx: &mut Context, panel: Option<Rect>) {
    let want = panel.map(|p| (p.x, p.y, p.width, p.height));
    if want == ctx.toast_view.toast_window {
        return;
    }
    ctx.toast_view.toast_window = want;
    let rid = ctx.issue_request_id();
    let _ = wm_client::window_close(ctx.ports.wm_port, rid, TOAST_WINDOW_ID);
    if let Some(p) = panel {
        let rid = ctx.issue_request_id();
        let _ = wm_client::window_open(
            ctx.ports.wm_port,
            rid,
            TOAST_WINDOW_ID,
            WINDOW_KIND_POPUP,
            (p.x, p.y, p.width, p.height),
        );
    }
}

const WINDOW_KIND_POPUP: u32 = 3;
