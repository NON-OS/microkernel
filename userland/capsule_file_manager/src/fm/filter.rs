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

use nonos_app_skeleton::input::text::{is_paste, typed_char};
use nonos_app_skeleton::{EventOutcome, InputEvent, KEY_BACKSPACE, KEY_ENTER, KEY_ESC};

use super::field_paste::paste_field;
use super::state::{Mode, State};
use super::text_field::{name_char, push_within, FILTER_MAX};
use super::view::rebuild_view;

// Live incremental search: each keystroke edits the filter and rebuilds the
// view immediately. Escape clears it, Enter keeps it and returns to browsing.
pub fn on_key(state: &mut State, event: InputEvent) -> EventOutcome {
    if is_paste(&event) {
        let refused = b"paste refused: a filter takes no spaces or tabs";
        if let Some(note) = paste_field(&mut state.filter, FILTER_MAX, name_char, refused) {
            state.status = note;
        }
        rebuild_view(state);
        return EventOutcome::Repaint;
    }
    match event.code {
        KEY_ESC => {
            state.filter.clear();
            state.mode = Mode::Browse;
            state.status = b"filter cleared";
            rebuild_view(state);
        }
        KEY_ENTER => {
            state.mode = Mode::Browse;
            state.status = b"click or Enter to open";
        }
        KEY_BACKSPACE => {
            state.filter.pop();
            rebuild_view(state);
        }
        _ => {
            let ch = typed_char(&event);
            if ch.is_some_and(|ch| push_within(&mut state.filter, ch, FILTER_MAX, name_char)) {
                rebuild_view(state);
            }
        }
    }
    EventOutcome::Repaint
}
