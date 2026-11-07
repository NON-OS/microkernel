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
//! The open menu's panel on screen: without its shadow, what a change of
//! the row under the pointer repaints; with it, what the menu opening or
//! going repaints.

use super::height::height;
use super::metrics::width;
use super::origin::origin;
use crate::render::layout::Rect;
use crate::render::panel::shadow_spread;
use crate::render::shadow_reach::with_shadow;
use crate::state::Context;

pub fn frame(ctx: &Context) -> Option<Rect> {
    ctx.desk_ui.desktop_menu?;
    let (x, y) = origin(ctx);
    Some(Rect { x, y, width: width(ctx), height: height(ctx) })
}

/// The panel and its shadow's rings (render/panel.rs shadow_panel), a pixel
/// more for the rounding, on the screen.
pub fn reach(ctx: &Context) -> Option<Rect> {
    let f = frame(ctx)?;
    let spread = shadow_spread().saturating_add(1);
    let (x, y, width, height) =
        with_shadow((f.x, f.y, f.width, f.height), spread, ctx.width, ctx.height);
    Some(Rect { x, y, width, height })
}
