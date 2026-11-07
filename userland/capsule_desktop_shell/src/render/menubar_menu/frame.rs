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
//! The open drop-down's panel on screen, without its shadow: what a change
//! of the row under the pointer repaints.

use super::metrics::{panel_h, panel_w};
use super::origin::origin;
use crate::render::layout::Rect;
use crate::render::panel::shadow_spread;
use crate::render::shadow_reach::with_shadow;
use crate::state::Context;

pub fn frame(ctx: &Context) -> Option<Rect> {
    let index = ctx.menubar.open?;
    let (x, y) = origin(ctx, index);
    Some(Rect { x, y, width: panel_w(ctx, index), height: panel_h(ctx, index) })
}

/// The open drop-down and its shadow's rings (render/panel.rs shadow_panel),
/// a pixel more for the rounding, on the screen: what its opening, closing
/// or going to another title changes besides the bar.
pub fn reach(ctx: &Context) -> Option<Rect> {
    let f = frame(ctx)?;
    let spread = shadow_spread().saturating_add(1);
    let (x, y, width, height) =
        with_shadow((f.x, f.y, f.width, f.height), spread, ctx.width, ctx.height);
    Some(Rect { x, y, width, height })
}
