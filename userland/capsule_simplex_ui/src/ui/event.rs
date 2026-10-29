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

//! Keys to the model. The window is used from the keyboard alone: Up and
//! Down move through the chats, Tab moves between the list and the compose
//! line, Ctrl+K searches, Enter sends, Ctrl+V pastes and Ctrl+C copies.

use nonos_app_skeleton::input::text::{is_paste, typed_char};
use nonos_app_skeleton::{
    clipboard_copy, clipboard_paste_line, EventOutcome, InputEvent, KEY_BACKSPACE, KEY_DOWN,
    KEY_ENTER, KEY_ESC, KEY_TAB, KEY_UP, MOD_CTRL,
};

use super::app::SimplexUi;
use crate::model::Key;

/// The paste buffer: a compose line's worth.
const PASTE_MAX: usize = crate::model::compose::MAX_COMPOSE;

fn key_of(event: &InputEvent) -> Option<Key> {
    let ctrl = event.flags & MOD_CTRL != 0;
    Some(match event.code {
        0x6B | 0x4B if ctrl => Key::Search,
        KEY_UP => Key::Up,
        KEY_DOWN => Key::Down,
        KEY_TAB => Key::Tab,
        KEY_ENTER => Key::Enter,
        KEY_ESC => Key::Esc,
        KEY_BACKSPACE => Key::Backspace,
        _ => Key::Char(typed_char(event)?),
    })
}

fn paste(app: &mut SimplexUi) -> bool {
    let mut buf = [0u8; PASTE_MAX];
    let Ok(Some(line)) = clipboard_paste_line(&mut buf) else { return false };
    app.model.insert(line.text);
    true
}

pub fn on_event(app: &mut SimplexUi, event: InputEvent) -> EventOutcome {
    if !event.is_key_down() {
        return EventOutcome::Idle;
    }
    let ctrl = event.flags & MOD_CTRL != 0;
    let changed = if is_paste(&event) {
        paste(app)
    } else if ctrl && matches!(event.code, 0x63 | 0x43) {
        if let Some(text) = app.model.copy_text() {
            let _ = clipboard_copy(text.as_bytes());
        }
        false
    } else {
        key_of(&event).is_some_and(|key| app.model.on_key(key))
    };
    if app.pump() || changed {
        EventOutcome::Repaint
    } else {
        EventOutcome::Idle
    }
}
