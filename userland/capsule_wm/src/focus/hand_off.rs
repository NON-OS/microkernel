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

//! Where keyboard focus goes when the window holding it closes, is minimised
//! or its process dies: to the window on top of what is still showing.
//!
//! It went nowhere. Focus was cleared, and the input router, told no window
//! had focus, sent each key to the last process it had delivered one to: the
//! window just minimised (a terminal typed into unseen), or the app that had
//! just closed its window, which dropped it. Typing reached the window the
//! person was looking at only after a click on it.

use super::FocusModel;
use crate::window::{Kind, Visibility, Window, WindowTable};

/// Whether a window can be handed focus it did not ask for: a normal window
/// or a dialog, on screen. A popup is not (the desktop shell's dock and toasts
/// are popups, and a menu takes focus by being pressed), nor is a tooltip.
fn takes_handed_focus(w: &Window) -> bool {
    w.visibility == Visibility::Visible && matches!(w.kind, Kind::Normal | Kind::Dialog)
}

/// The window keyboard focus belongs on when nothing chose one: the highest
/// in the stack of those `takes_handed_focus` allows.
pub fn next_focus(table: &WindowTable) -> Option<(u32, u32)> {
    table
        .windows()
        .filter(|w| takes_handed_focus(w))
        .max_by_key(|w| w.z)
        .map(|w| (w.owner_pid, w.window_id))
}

/// Move focus on when the window holding it is gone from the table or no
/// longer showing. Returns the pid the compositor must be told has focus now
/// (0 when no window is left to take it), or None when focus did not change.
pub fn hand_off(table: &WindowTable, focus: &mut FocusModel) -> Option<u32> {
    if let Some(f) = focus.current() {
        let showing = table.find(f.owner_pid, f.window_id);
        if showing.is_some_and(|w| w.visibility == Visibility::Visible) {
            return None;
        }
    }
    match next_focus(table) {
        Some((pid, window_id)) => focus.set(pid, window_id).then_some(pid),
        None => focus.clear().then_some(0),
    }
}
