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

use crate::state::Context;

// Clients submit layer rectangles unclipped, so damage can hang off the edge.
// The kernel rejects a rectangle that leaves the framebuffer, and a rejected
// present takes the compositor down, so trim here. None means nothing visible.
pub fn clip_to_screen(
    ctx: &Context,
    rect: crate::state::damage::Rect,
) -> Option<crate::state::damage::Rect> {
    let screen = ctx.screen;
    if rect.x >= screen.width || rect.y >= screen.height {
        return None;
    }
    let width = core::cmp::min(rect.width, screen.width - rect.x);
    let height = core::cmp::min(rect.height, screen.height - rect.y);
    if width == 0 || height == 0 {
        return None;
    }
    Some(crate::state::damage::Rect { x: rect.x, y: rect.y, width, height })
}
