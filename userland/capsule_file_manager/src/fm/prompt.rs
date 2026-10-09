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

use alloc::string::String;

use nonos_app_skeleton::input::text::{is_paste, typed_char};
use nonos_app_skeleton::{EventOutcome, InputEvent, KEY_BACKSPACE, KEY_ENTER, KEY_ESC};

use super::field_paste::paste_field;
use super::prompt_commit::commit;
use super::state::{Mode, State};
use super::text_field::{name_char, push_within, NAME_MAX};

pub fn on_key(state: &mut State, event: InputEvent) -> EventOutcome {
    let Mode::Prompt(kind) = state.mode else { return EventOutcome::Idle };
    if is_paste(&event) {
        let refused = b"paste refused: a name takes no spaces or tabs";
        if let Some(note) = paste_field(&mut state.input, NAME_MAX, name_char, refused) {
            state.status = note;
        }
        return EventOutcome::Repaint;
    }
    match event.code {
        KEY_ESC => {
            state.mode = Mode::Browse;
            state.input = String::new();
            state.status = b"cancelled";
        }
        KEY_BACKSPACE => {
            state.input.pop();
        }
        KEY_ENTER => commit(state, kind),
        _ => {
            if let Some(ch) = typed_char(&event) {
                push_within(&mut state.input, ch, NAME_MAX, name_char);
            }
        }
    }
    EventOutcome::Repaint
}
