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

use super::types::TaskbarState;

/// How long a press on the brand shows the hidden dock, unless the pointer
/// goes to it.
pub(super) const BRAND_REVEAL_MS: i64 = 1800;

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
