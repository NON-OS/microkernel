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

use crate::compositor_client::push_damage_commit;
use crate::render::layout::dock_area_rect;
use crate::render::{paint_chrome_in, Changed};
use crate::state::Context;

/// Repaint the chrome and present the dock's whole area, panel and shadow
/// (render/layout.rs dock_area_rect, through render/shadow_reach.rs), whether
/// the dock is drawn there or the area was cleared to let the full-screen
/// window under it show, so a hidden dock leaves no outline behind. The
/// frame is shown with only that area composed again (render/chrome/show.rs),
/// and the toasts' area when toasts not yet drawn go on it; the lock screen
/// has it shown whole. A change of the menubar's app shows the bar too
/// (refresh_after).
pub fn refresh_taskbar(ctx: &mut Context) {
    let r = dock_area_rect(ctx.width, ctx.height);
    // The dock's own frame: a show or hide is in its area.
    match super::repaint::owed_parts(ctx, &[(r.x, r.y, r.width, r.height)], false) {
        Some(parts) => super::repaint::show_parts(ctx, parts),
        None => {
            let dock = [(r.x, r.y, r.width, r.height)];
            for &(x, y, w, h) in paint_chrome_in(ctx, Changed::Whole).owed().of(&dock) {
                let rid = ctx.issue_request_id();
                let _ = push_damage_commit(ctx.ports.compositor_port, rid, x, y, w, h);
            }
        }
    }
    ctx.taskbar.drawn = ctx.taskbar.visible;
}

/// Show a change to the dock that may also have moved the active app, which
/// was `named` before it: the menubar names the active app, so when it moved
/// the bar is shown again with the dock's area (repaint.rs repaint_focus),
/// and otherwise the dock's area alone. A window closed, or forgotten
/// because its process is gone, can move it.
pub fn refresh_after(ctx: &mut Context, named: u8) {
    if ctx.taskbar.active != named {
        super::repaint::repaint_focus(ctx);
    } else {
        refresh_taskbar(ctx);
    }
}
