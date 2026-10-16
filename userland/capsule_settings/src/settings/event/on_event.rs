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

use super::on_event_browsing::on_event_browsing;
use super::on_event_editing::on_event_editing;
use super::on_event_wifi::on_event_wifi;
use super::on_pointer::on_pointer;
use super::on_search_key::on_search_key;
use super::paste::on_paste;
use super::wheel::wheel;
use crate::settings::section::Section;
use crate::settings::state::State;
use nonos_app_skeleton::input::text::is_paste;
use nonos_app_skeleton::{EventOutcome, InputEvent, InputKind, MOD_ALT, MOD_CTRL, MOD_META};

pub fn on_event(state: &mut State, event: InputEvent) -> EventOutcome {
    if event.kind == InputKind::ButtonDown {
        return on_pointer(state, event.x, event.y);
    }
    if event.kind == InputKind::Wheel && !state.editing {
        return wheel(state, event.delta_y);
    }
    if !event.is_key_down() {
        return EventOutcome::Idle;
    }
    // Ctrl+V or Shift+Insert into whichever field has the keyboard.
    if is_paste(&event) {
        if let Some(out) = on_paste(state) {
            return out;
        }
    }
    if state.search.focused {
        return on_search_key(state, &event);
    }
    if state.editing {
        // A chord is a command, and the editor's only one is paste: Ctrl with
        // a letter typed the letter into the value.
        if event.flags & (MOD_CTRL | MOD_ALT | MOD_META) != 0 {
            return EventOutcome::Idle;
        }
        return on_event_editing(state, event.code);
    }
    if state.section == Section::Wifi {
        return on_event_wifi(state, event.code);
    }
    on_event_browsing(state, event.code)
}
