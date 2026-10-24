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

use super::paint::paint_toasts;
use super::place::{panel_rect, toast_rect};
use super::press::claim_presses;
use crate::compositor_client::push_damage_commit;
use crate::state::Context;

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
    ctx.toast_view.toasts_synced = ctx.toasts.generation();
    let live = !ctx.toasts.is_empty();
    if !live && !ctx.toast_view.toast_layer_live {
        return;
    }
    if ctx.toast_view.toasts_drawn != ctx.toasts.generation() {
        crate::render::paint_chrome(ctx);
    }
    let r = toast_rect(ctx.width, ctx.height);
    ctx.toast_view.toast_layer_live = live;
    let panel = live.then(|| panel_rect(ctx, r));
    claim_presses(ctx, panel);
    let rid = ctx.issue_request_id();
    let _ = push_damage_commit(ctx.ports.compositor_port, rid, r.x, r.y, r.width, r.height);
}

/// The toasts, last in the chrome's frame so they sit over everything in it.
pub fn paint_in_chrome(ctx: &mut Context) {
    ctx.toast_view.toasts_drawn = ctx.toasts.generation();
    if !ctx.toasts.is_empty() {
        paint_toasts(ctx, toast_rect(ctx.width, ctx.height));
    }
}
