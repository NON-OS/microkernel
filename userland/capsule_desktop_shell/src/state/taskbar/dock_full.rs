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

//! The full-screen windows that hide the dock, as the window manager
//! reports them and as their processes end.

use super::dock_rule::settle;
use super::types::TaskbarState;

/// Cap on remembered full-screen windows; the window table holds fewer.
const FULL_SCREEN_MAX: usize = 64;

/// The window manager says `pid`'s window `window_id` covers the dock's band
/// (`on`) or no longer does. Returns true when the dock is to be shown or
/// hidden for it.
pub fn set_full_screen(state: &mut TaskbarState, pid: u32, window_id: u32, on: bool) -> bool {
    state.full_screen.retain(|&(p, w)| !(p == pid && w == window_id));
    if on {
        if state.full_screen.len() >= FULL_SCREEN_MAX {
            state.full_screen.remove(0);
        }
        state.full_screen.push((pid, window_id));
    }
    settle(state)
}

/// Forget the full-screen windows whose process `alive` says has ended, as
/// if their close had come. Returns true when the dock is to be shown.
pub fn forget_dead_full_screen(state: &mut TaskbarState, alive: impl Fn(u32) -> bool) -> bool {
    state.full_screen.retain(|&(pid, _)| alive(pid));
    settle(state)
}
