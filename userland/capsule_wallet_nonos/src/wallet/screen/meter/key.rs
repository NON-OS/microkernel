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

use super::budget::FIELD_CAP;
use super::channels::FIELD_TOPUP;
use crate::wallet::meter::budget_form::push_digit;
use crate::wallet::state::{State, VIEW_METER};

/// A key on the meter screen: digits into the focused field, Tab to the
/// next, Backspace, and Enter to set the budget or top up; on the purchase
/// screen, its channel form's.
pub fn key(state: &mut State, code: u32) -> Option<EventOutcome> {
    if state.view != VIEW_METER {
        // The purchase screen's voucher form, the only other meter form.
        return crate::wallet::screen::purchase::form::key(state, code);
    }
    let m = &mut state.meter;
    let fields = if m.picked.is_some() { 3 } else { 2 };
    let (text, max) = match m.focus {
        FIELD_CAP => (&mut m.cap_text, 13),
        FIELD_TOPUP => (&mut m.topup_text, 13),
        _ => (&mut m.days_text, 3),
    };
    match code {
        KEY_TAB => m.focus = (m.focus + 1) % fields,
        KEY_ENTER if m.focus == FIELD_TOPUP => return Some(super::jobs::topup(state)),
        KEY_ENTER => return Some(super::budget::set(state)),
        KEY_BACKSPACE => {
            text.pop();
        }
        c if push_digit(text, c, max) => {}
        _ => return None,
    }
    state.failure = None;
    Some(EventOutcome::Repaint)
}
