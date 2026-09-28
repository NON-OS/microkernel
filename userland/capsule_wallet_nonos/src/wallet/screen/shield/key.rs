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
 * Typing on the private send screen: Tab moves between the fields shown,
 * Backspace deletes, Enter opens the review, and each field keeps only
 * what it can hold. Other Shield screens take no typed input.
 */

use nonos_app_skeleton::{EventOutcome, KEY_BACKSPACE, KEY_ENTER, KEY_TAB};

use crate::wallet::state::shield_ui::*;
use crate::wallet::state::{State, VIEW_SHIELD};

const MAX_TO: usize = 4096;
const MAX_SHORT: usize = 40;

pub fn key(state: &mut State, code: u32) -> Option<EventOutcome> {
    if state.view != VIEW_SHIELD || state.shield_ui.screen != SHIELD_SEND {
        return None;
    }
    let early = !super::meter::waited(&state.shield_ui);
    let ui = &mut state.shield_ui;
    match code {
        KEY_TAB => ui.focus = (ui.focus + 1) % if early { 3 } else { 2 },
        KEY_ENTER => return Some(super::footer::footer(state, 0)),
        KEY_BACKSPACE => {
            let _ = field(ui).pop();
        }
        c if (0x20..0x7F).contains(&c) => push(ui, c as u8 as char),
        _ => return None,
    }
    Some(EventOutcome::Repaint)
}

fn field(ui: &mut ShieldUi) -> &mut alloc::string::String {
    match ui.focus {
        FIELD_AMOUNT => &mut ui.amount,
        FIELD_OVERRIDE => &mut ui.override_text,
        _ => &mut ui.to,
    }
}

pub fn push(ui: &mut ShieldUi, c: char) {
    let (keep, max) = match ui.focus {
        FIELD_AMOUNT => (c.is_ascii_digit() || c == '.', MAX_SHORT),
        FIELD_OVERRIDE => (true, MAX_SHORT),
        _ => (c.is_ascii_alphanumeric(), MAX_TO),
    };
    /* A bech32 address is written in lower case; the others keep case. */
    let lower = ui.focus == FIELD_TO;
    let f = field(ui);
    if keep && f.len() < max {
        f.push(if lower { c.to_ascii_lowercase() } else { c });
    }
}
