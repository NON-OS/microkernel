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
use crate::render::paint_chrome;
use crate::state::Context;

/// Repaint the chrome and present the dock's whole area, panel and shadow
/// (render/layout.rs dock_area_rect, through render/shadow_reach.rs), whether
/// the dock is drawn there or the area was cleared to let the full-screen
/// window under it show, so a hidden dock leaves no outline behind.
pub fn refresh_taskbar(ctx: &mut Context) {
    paint_chrome(ctx);
    let r = dock_area_rect(ctx.width, ctx.height);
    let rid = ctx.issue_request_id();
    let _ = push_damage_commit(ctx.compositor_port, rid, r.x, r.y, r.width, r.height);
    ctx.taskbar.drawn = ctx.taskbar.visible;
}
