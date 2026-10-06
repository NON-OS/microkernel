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

//! Input.

use nonos_app_skeleton::{EventOutcome, InputEvent, InputKind, KEY_ESC};

use super::event_click::on_click;
use super::event_keys::on_key;
use super::state::State;

pub fn on_event(state: &mut State, event: InputEvent) -> EventOutcome {
    if event.kind == InputKind::ButtonDown {
        return on_click(state, event.x, event.y);
    }
    // A wheel travels the list and leaves the selection alone.
    if event.kind == InputKind::Wheel {
        let rows = -(event.delta_y.signum() as isize) * 3;
        return match state.scroll_by(rows) {
            true => EventOutcome::Repaint,
            false => EventOutcome::Idle,
        };
    }
    if !event.is_key_down() {
        return EventOutcome::Idle;
    }
    /*
     * Escape closes the window, unless the search field has it: see
     * `event_search`, which gives it back.
     */
    if event.code == KEY_ESC && !state.search.active {
        return EventOutcome::Close;
    }
    on_key(state, event.code)
}
