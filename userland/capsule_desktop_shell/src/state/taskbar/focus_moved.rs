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
//! The menu bar names the app whose window has focus, and the dock marks
//! it: on every focus the window manager reports (`WmEvent::Focused`), not
//! only when a window opens or closes. Pure; held in desktop_proofs.

use super::types::{TaskbarState, TASKBAR_NO_ACTIVE};

/// `app` is the launcher index of the focused window's app, None when
/// nothing has focus or the window is no launcher app's ("Desktop"). True
/// when the named app changes.
pub fn focus_moved(state: &mut TaskbarState, app: Option<usize>) -> bool {
    let next = match app {
        Some(i) if i < state.open.len() => i as u8,
        _ => TASKBAR_NO_ACTIVE,
    };
    if state.active == next {
        return false;
    }
    state.active = next;
    true
}
