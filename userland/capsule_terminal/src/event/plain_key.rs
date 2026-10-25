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

//! The keys that edit, recall or type into the line.

use nonos_app_skeleton::input::text::typed_char;
use nonos_app_skeleton::{
    EventOutcome, InputEvent, KEY_BACKSPACE, KEY_DELETE, KEY_DOWN, KEY_ENTER, KEY_ESC, KEY_TAB,
    KEY_UP,
};

use crate::event::bool_to_outcome::bool_to_outcome;
use crate::event::on_down::on_down;
use crate::event::on_enter::on_enter;
use crate::event::on_printable::on_printable;
use crate::event::on_tab::on_tab;
use crate::event::on_up::on_up;
use crate::term::state::State;

/// A key no earlier handler took: the line's own editing keys, history,
/// completion, or a character to type.
pub(super) fn plain_key(state: &mut State, event: InputEvent) -> EventOutcome {
    match event.code {
        // Clears the line. Esc used to close the window, so one stray press
        // took the scrollback, history and working directory with it. Closing
        // is the titlebar's job.
        KEY_ESC => {
            state.line.clear();
            EventOutcome::Repaint
        }
        KEY_ENTER => on_enter(state),
        KEY_BACKSPACE if state.search.is_some() => {
            super::super::search_edit::search_backspace(state);
            EventOutcome::Repaint
        }
        KEY_BACKSPACE => bool_to_outcome(state.line.backspace()),
        KEY_DELETE => bool_to_outcome(state.line.delete()),
        KEY_UP => on_up(state),
        KEY_DOWN => on_down(state),
        KEY_TAB => on_tab(state),
        _ => match typed_char(&event) {
            // Any character the keymap typed, not only ASCII: the line holds
            // UTF-8 and draws what the font has, a box for what it lacks.
            Some(ch) => on_printable(state, ch),
            None => EventOutcome::Idle,
        },
    }
}
