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

//! Scrolling an Etna screen.

use nonos_app_skeleton::EventOutcome;

use crate::wallet::screen::hits::limit;
use crate::wallet::state::State;

/// The wheel moves the photograph and content under the fixed bar and foot,
/// no further than the screen's content reaches.
pub fn scroll(state: &mut State, delta_y: i32) -> EventOutcome {
    let next = (i64::from(state.scroll) - i64::from(delta_y) * 60).clamp(0, i64::from(limit()));
    state.scroll = next as u32;
    EventOutcome::Repaint
}

/// The arrow and page keys scroll an Etna screen as the wheel does: one
/// wheel step for an arrow, five for a page.
pub fn scroll_key(state: &mut State, code: u32) -> Option<EventOutcome> {
    use nonos_app_skeleton::{KEY_DOWN, KEY_PAGE_DOWN, KEY_PAGE_UP, KEY_UP};
    let steps = match code {
        KEY_UP => 1,
        KEY_DOWN => -1,
        KEY_PAGE_UP => 5,
        KEY_PAGE_DOWN => -5,
        _ => return None,
    };
    Some(scroll(state, steps))
}
