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
//! Where the dragged icon is drawn under the pointer (paint.rs
//! paint_drag_ghost): what a step of the drag repaints, where it was and
//! where it is.

use super::metrics::icon;
use crate::render::layout::Rect;
use crate::state::Context;

pub fn ghost_rect(ctx: &Context) -> Option<Rect> {
    if !ctx.desk_ui.drag_moved {
        return None;
    }
    ctx.desk_ui.drag_from.and_then(|i| ctx.desktop_items.get(i))?;
    let x = ctx.desk_ui.drag_x.saturating_sub(icon() / 2);
    let y = ctx.desk_ui.drag_y.saturating_sub(icon() / 2);
    Some(Rect { x, y, width: icon(), height: icon() })
}
