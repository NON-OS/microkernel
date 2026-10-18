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

//! The dock's reveal from the pointer's height.

use crate::render::layout::dock_area_rect;
use crate::state::{dock_pointer, Context};

/// The pointer, mirrored to the shell by the input router wherever it is,
/// over windows too: touching the bottom edge brings the dock hidden by a
/// full-screen window, and leaving the dock's area puts it away again
/// (state/taskbar/dock_rule.rs). The dock's sync after this batch draws the
/// change and opens or closes its window.
pub(super) fn hover_reveal(ctx: &mut Context, y: u32) {
    if ctx.height == 0 {
        return;
    }
    let top = dock_area_rect(ctx.width, ctx.height).y;
    dock_pointer(&mut ctx.taskbar, y, ctx.height, top);
}
