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

//! What the shell still owes for the dock: a repaint, and its window opened
//! or closed.

use super::types::TaskbarState;

/// What the shell still owes the screen and the window manager for the dock
/// as the rule has it: a repaint of its area (`paint`) and the popup window
/// opened or closed (`window`). The shell does them and marks them done.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DockWork {
    pub paint: bool,
    pub window: Option<bool>,
}

pub fn dock_work(state: &TaskbarState) -> DockWork {
    DockWork {
        paint: state.drawn != state.visible,
        window: (state.window_open != state.visible).then_some(state.visible),
    }
}
