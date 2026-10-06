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

//! When the dock is shown: always, except while a full-screen window shows.
//!
//! The green button fills the display down to its bottom edge, the band the
//! dock is drawn in. While any window the window manager says covers that
//! band (made full screen and not minimised, its kind 2 notification) is
//! up, the dock is hidden: nothing is drawn there and its popup window is
//! closed, so the window shows whole and a press at the bottom reaches it.
//! The pointer touching the bottom edge brings the dock over the window
//! (`dock_pointer`), and it hides again when the pointer leaves the dock's
//! area. A press on the menubar's brand brings it for 1.8 s, or for as long
//! as the pointer is then in its area. The window restored by green,
//! minimised or closed, or its process ending, gives the dock back.
//!
//! Before, the dock was always shown and the green button stopped above its
//! band, so a maximised window left a band of desktop with the dock in it.

use super::types::{TaskbarState, Uptime};

/// Rows at the very bottom of the screen a pointer touches to bring the
/// hidden dock back.
pub const HOVER_REVEAL_BAND: u32 = 4;

/// How long a press on the brand shows the hidden dock, unless the pointer
/// goes to it.
pub(super) const BRAND_REVEAL_MS: i64 = 1800;

/// Cap on remembered full-screen windows; the window table holds fewer.
const FULL_SCREEN_MAX: usize = 64;

/// Whether a full-screen window is up, so the dock is hidden unless revealed.
pub fn dock_covered(state: &TaskbarState) -> bool {
    !state.full_screen.is_empty()
}

/// Bring `visible` in line with the rule. Returns true when it changed.
pub(super) fn settle(state: &mut TaskbarState) -> bool {
    let covered = dock_covered(state);
    if !covered {
        state.revealed = false;
        state.reveal_until_ms = 0;
    }
    let shown = !covered || state.revealed;
    let changed = shown != state.visible;
    state.visible = shown;
    changed
}

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

/// The pointer at row `y` of a `height` row screen, with the dock's area
/// (its panel and shadow, the full width down to the bottom edge) from row
/// `dock_top`. Touching the bottom edge brings the hidden dock; leaving its
/// area puts it away again. Returns true when the dock is to be shown or
/// hidden.
pub fn dock_pointer(state: &mut TaskbarState, y: u32, height: u32, dock_top: u32) -> bool {
    state.pointer_in_dock = y >= dock_top;
    if dock_covered(state) && y >= height.saturating_sub(HOVER_REVEAL_BAND) {
        state.revealed = true;
    }
    if state.pointer_in_dock {
        // The pointer holds it now, whatever brought it.
        state.reveal_until_ms = 0;
    } else if state.revealed && state.reveal_until_ms == 0 {
        state.revealed = false;
    }
    settle(state)
}

/// A press on the brand, or the Go menu's Show Dock: the hidden dock for
/// 1.8 s, or for as long as the pointer is then in its area. Returns true
/// when the dock is to be shown.
pub fn reveal_taskbar(state: &mut TaskbarState, now: Uptime) -> bool {
    if dock_covered(state) {
        state.revealed = true;
        if !state.pointer_in_dock {
            state.reveal_until_ms = now.0.saturating_add(BRAND_REVEAL_MS).max(1);
        }
    }
    settle(state)
}

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
