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

//! The hidden dock brought back: by the pointer at the bottom edge, or for
//! a while by a press on the brand.

use super::dock_rule::{dock_covered, settle, BRAND_REVEAL_MS};
use super::types::{TaskbarState, Uptime};

/// Rows at the very bottom of the screen a pointer touches to bring the
/// hidden dock back.
pub const HOVER_REVEAL_BAND: u32 = 4;

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
