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

//! Ctrl+V and Shift+Insert: a copied number or sum, entered as its keys.

use nonos_app_skeleton::{clipboard_paste_line, EventOutcome};

use crate::calc::actions::dispatch;
use crate::calc::buttons::Action;
use crate::calc::mode::Mode;
use crate::calc::paste::{keys_for, PasteKey};
use crate::calc::state::State;

pub fn on_paste(state: &mut State) -> EventOutcome {
    if state.mode == Mode::History {
        return EventOutcome::Idle;
    }
    let mut buf = [0u8; 256];
    let Ok(Some(line)) = clipboard_paste_line(&mut buf) else {
        return EventOutcome::Idle;
    };
    let radix = (state.mode == Mode::Programmer).then(|| state.base.radix());
    // Text that is not a number or a sum changes nothing: the display keeps
    // what was on it rather than part of what was copied.
    let Some(keys) = keys_for(line.text, radix) else {
        return EventOutcome::Idle;
    };
    // A paste is a fresh entry. After an error it starts over, as a press of
    // C would; otherwise it replaces the number being typed, and a pending
    // operator stays pending, so "12 +" then a pasted 30 gives 42.
    if state.is_error() {
        dispatch::run(state, Action::Clear);
    }
    state.reset_input();
    for key in keys {
        dispatch::run(state, action_of(key));
    }
    EventOutcome::Repaint
}

fn action_of(key: PasteKey) -> Action {
    match key {
        PasteKey::Digit(d) => Action::Digit(d),
        PasteKey::Point => Action::Decimal,
        PasteKey::Op(op) => Action::Operator(op),
        PasteKey::Negate => Action::Negate,
        PasteKey::Equals => Action::Equals,
    }
}
