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

use nonos_app_skeleton::input::text::is_paste;
use nonos_app_skeleton::{EventOutcome, InputEvent, MOD_CTRL};

use super::on_ctrl::on_ctrl;
use crate::term::state::State;

#[path = "plain_key.rs"]
mod plain_key;
use plain_key::plain_key;

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
    plain_key(state, event)
}
