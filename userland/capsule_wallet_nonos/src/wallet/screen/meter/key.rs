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

use nonos_app_skeleton::{EventOutcome, KEY_BACKSPACE, KEY_ENTER, KEY_TAB};

use super::budget::{FIELD_CAP, FIELD_DAYS};
use crate::wallet::meter::budget_form::push_digit;
use crate::wallet::state::{State, VIEW_METER};

/// A key on the meter screen: digits into the focused field, Tab to the
/// other, Backspace, and Enter to set the budget.
pub fn key(state: &mut State, code: u32) -> Option<EventOutcome> {
    if state.view != VIEW_METER {
        return None;
    }
    let m = &mut state.meter;
    let (text, max) =
        if m.focus == FIELD_CAP { (&mut m.cap_text, 13) } else { (&mut m.days_text, 3) };
    match code {
        KEY_TAB => m.focus = if m.focus == FIELD_CAP { FIELD_DAYS } else { FIELD_CAP },
        KEY_ENTER => return Some(super::click::set(state)),
        KEY_BACKSPACE => {
            text.pop();
        }
        c if push_digit(text, c, max) => {}
        _ => return None,
    }
    state.failure = None;
    Some(EventOutcome::Repaint)
}
