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
//! What one drain of the inbox asks of the frame loop.

use crate::setup::WindowBinding;

#[derive(Default)]
pub(in crate::runner) struct DrainResult {
    pub repaint: bool,
    pub close: bool,
    pub minimize: bool,
    pub maximize: bool,
    pub restore: bool,
    pub move_to: Option<(u32, u32)>,
    pub resize_to: Option<(u32, u32)>,
    /// A drag of the left or top edge: origin and size together.
    pub place: Option<nonos_toolkit::decorations::Placed>,
    /// The WM said the window took (true) or lost (false) the keyboard.
    pub focused: Option<bool>,
}

/// The window's size, place and state, as one drain sees it.
#[derive(Clone, Copy)]
pub(in crate::runner) struct Frame {
    pub width: u32,
    pub height: u32,
    pub win_x: u32,
    pub win_y: u32,
    pub maximized: bool,
}

impl Frame {
    /// The window `binding` shows, maximised or not.
    pub fn of(binding: &WindowBinding, maximized: bool) -> Self {
        let b = binding;
        Frame { width: b.width, height: b.height, win_x: b.x, win_y: b.y, maximized }
    }
}
