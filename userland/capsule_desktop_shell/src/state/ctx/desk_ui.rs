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

//! The desktop's gestures: the right-click menu and what it acts on, an
//! inline rename, and an icon being dragged.

pub struct DeskUi {
    /// Top-left corner of the desktop right-click menu, or None when hidden.
    pub desktop_menu: Option<(u32, u32)>,
    /// Which menu row the pointer is over, so it can be highlighted.
    pub menu_hover: Option<usize>,
    /// The desktop item the open menu acts on; None means the empty-desktop
    /// menu (New Folder / New File) rather than the per-item one.
    pub menu_target: Option<usize>,
    /// The item being renamed and its edit buffer, while an inline rename is in
    /// progress.
    pub rename: Option<usize>,
    pub rename_buf: alloc::string::String,
    /// Drag-to-move: the icon a left-press started on, whether the pointer has
    /// moved far enough to count as a drag, and the current pointer position so
    /// the dragged icon can follow the cursor.
    pub drag_from: Option<usize>,
    pub drag_moved: bool,
    pub drag_x: u32,
    pub drag_y: u32,
}

impl Default for DeskUi {
    fn default() -> Self {
        Self::new()
    }
}

impl DeskUi {
    pub fn new() -> Self {
        DeskUi {
            desktop_menu: None,
            menu_hover: None,
            menu_target: None,
            rename: None,
            rename_buf: alloc::string::String::new(),
            drag_from: None,
            drag_moved: false,
            drag_x: 0,
            drag_y: 0,
        }
    }
}
