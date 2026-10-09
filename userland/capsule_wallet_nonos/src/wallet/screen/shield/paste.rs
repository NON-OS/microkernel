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

/*
 * Ctrl+V on the private send screen. A nox1 address carries the public
 * keys a sender needs and is far too long to type, so pasting is the way
 * one arrives: a paste that carries an address goes to To, whichever field
 * has the keyboard, and replaces what was there. Anything else lands in
 * the focused field. Each character still passes the field's own filter.
 */

use nonos_app_skeleton::clients::clipboard::clipboard_paste;
use nonos_app_skeleton::EventOutcome;

use crate::wallet::state::shield_ui::{FIELD_TO, SHIELD_SEND};
use crate::wallet::state::{State, VIEW_SHIELD};

const MAX_PASTE: usize = 4096;

pub fn paste(state: &mut State) -> Option<EventOutcome> {
    if state.view != VIEW_SHIELD || state.shield_ui.screen != SHIELD_SEND {
        return None;
    }
    let mut buf = alloc::vec![0u8; MAX_PASTE];
    let n = clipboard_paste(&mut buf).ok()?;
    let text = core::str::from_utf8(&buf[..n.min(MAX_PASTE)]).ok()?;
    let ui = &mut state.shield_ui;
    let text = match super::typed::pasted_address(text) {
        Some(address) => {
            ui.focus = FIELD_TO;
            ui.to.clear();
            address
        }
        None => text.trim(),
    };
    for c in text.chars() {
        super::key::push(ui, c);
    }
    Some(EventOutcome::Repaint)
}
