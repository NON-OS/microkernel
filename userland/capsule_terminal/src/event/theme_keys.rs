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

//! Keys while the theme picker is open (`term/theme_pick.rs`): each move puts
//! a theme on the window at once, through the same request `theme <name>` makes.

use nonos_app_skeleton::{EventOutcome, KEY_DOWN, KEY_ENTER, KEY_ESC, KEY_UP};

use crate::term::state::State;
use crate::term::theme::profiles::COUNT;
use crate::term::theme_pick::{Key, Then};

pub fn key(state: &mut State, code: u32) -> EventOutcome {
    let Some(mut pick) = state.theme_pick else { return EventOutcome::Idle };
    let k = match code {
        KEY_UP => Key::Up,
        KEY_DOWN => Key::Down,
        KEY_ENTER => Key::Enter,
        KEY_ESC => Key::Esc,
        _ => Key::Other,
    };
    match pick.key(k, COUNT) {
        Then::Show(i) => {
            state.theme_req = Some(i);
            state.theme_pick = Some(pick);
        }
        Then::Close(i) => {
            state.theme_req = Some(i);
            state.theme_pick = None;
        }
        Then::Stay => return EventOutcome::Idle,
    }
    EventOutcome::Repaint
}
