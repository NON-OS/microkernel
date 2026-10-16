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

use super::search_results::{open_selected, step};
use crate::settings::state::cache::STRING_CAP;
use crate::settings::state::{search_clear, State};
use nonos_app_skeleton::input::text::typed_char;
use nonos_app_skeleton::{
    EventOutcome, InputEvent, KEY_BACKSPACE, KEY_DOWN, KEY_ENTER, KEY_ESC, KEY_UP,
};

/// Widest query the titlebar field can show without the text running under its
/// rounded right edge, so typing stops rather than scrolling out of sight.
/// Counted in characters: the field draws characters, whatever their bytes.
pub(super) const QUERY_MAX: usize = 24;

pub(super) fn on_search_key(state: &mut State, event: &InputEvent) -> EventOutcome {
    match event.code {
        KEY_ESC => {
            search_clear(state);
            state.search.focused = false;
            EventOutcome::Repaint
        }
        KEY_BACKSPACE => {
            state.search.text.pop();
            state.search.cursor = 0;
            state.search.scroll = 0;
            EventOutcome::Repaint
        }
        KEY_UP => step(state, -1),
        KEY_DOWN => step(state, 1),
        KEY_ENTER => open_selected(state),
        _ => match typed_char(event) {
            Some(ch) if state.search.text.char_count() < QUERY_MAX => {
                if !state.search.text.push_char(ch, STRING_CAP) {
                    return EventOutcome::Idle;
                }
                state.search.cursor = 0;
                state.search.scroll = 0;
                EventOutcome::Repaint
            }
            _ => EventOutcome::Idle,
        },
    }
}
