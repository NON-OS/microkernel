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
use nonos_app_skeleton::{
    EventOutcome, InputEvent, KEY_BACKSPACE, KEY_DELETE, KEY_DOWN, KEY_ENTER, KEY_ESC, KEY_TAB,
    KEY_UP, MOD_CTRL,
};

use super::bool_to_outcome::bool_to_outcome;
use super::on_ctrl::on_ctrl;
use super::on_down::on_down;
use super::on_enter::on_enter;
use super::on_printable::on_printable;
use super::on_tab::on_tab;
use super::on_up::on_up;
use crate::term::state::State;

pub fn on_key(state: &mut State, event: InputEvent) -> EventOutcome {
    if let Some(out) = super::key_first::key_first(state, event) {
        return out;
    }
    // Shift+Insert pastes as Ctrl+V does; Ctrl+V itself is on_ctrl's.
    if is_paste(&event) && event.flags & MOD_CTRL == 0 {
        return super::paste_clipboard::paste_clipboard(state);
    }
    if event.flags & MOD_CTRL != 0 {
        if let Some(out) = on_ctrl(state, event.code, event.flags) {
            return out;
        }
    }
    if let Some(out) = super::readline::readline_key(state, event.code, event.flags) {
        return out;
    }
    if let Some(out) = super::on_nav::on_nav(state, event.code) {
        return out;
    }
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
            super::search_edit::search_backspace(state);
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
