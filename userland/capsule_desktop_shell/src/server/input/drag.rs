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

//! A drag's release: onto a folder it moves the item, unmoved it opens it.

use super::open_item::open_item;
use crate::render::desktop_icons;
use crate::server::desktop;
use crate::state::Context;

// Resolve a drag on release: dropped onto a folder it moves the file there, a
// press that never moved is a click that opens the item. Always lets go of the
// pointer grab and the drag state, so nothing stays stuck to the cursor.
pub(super) fn drop_drag(ctx: &mut Context) {
    let Some(src) = ctx.desk_ui.drag_from.take() else {
        return;
    };
    desktop::release_drag(ctx);
    if ctx.desk_ui.drag_moved {
        if let Some(target) = desktop_icons::hit(ctx, ctx.desk_ui.drag_x, ctx.desk_ui.drag_y) {
            desktop::move_into(ctx, src, target);
        }
    } else {
        open_item(ctx, src);
    }
    ctx.desk_ui.drag_moved = false;
    super::super::repaint::repaint(ctx);
}
