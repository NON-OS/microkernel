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

//! The budget form on the meter screen (B8): a cap in whole NOX and how
//! many days it runs. Set budget replaces the meter's budget whole.

use nonos_app_skeleton::PaintBuffer;

use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::GAP;
use crate::wallet::screen::hits::{self, Press};
use crate::wallet::screen::shield::parts::field;
use nonos_app_skeleton::EventOutcome;

use crate::wallet::state::State;

pub const FIELD_CAP: u8 = 0;
pub const FIELD_DAYS: u8 = 1;

pub fn form(state: &State, fb: &mut PaintBuffer, c: Rect, y: u32) -> u32 {
    let m = &state.meter;
    let mut h = 0;
    for (id, name, text, hint) in [
        (FIELD_CAP, "NEW BUDGET, WHOLE NOX", &m.cap_text, "for example 100"),
        (FIELD_DAYS, "FOR HOW MANY DAYS", &m.days_text, "1 to 365"),
    ] {
        let (at, used) = field(fb, c, y + h, name, text, hint, m.focus == id);
        hits::put(Press::Field(id), at);
        h += used + GAP / 2;
    }
    h + GAP / 2
}

/// Whether the meter has its key open, so it takes a budget.
pub fn ready(state: &State) -> bool {
    state.meter.account.as_ref().is_some_and(|a| a.flags & nonos_payment_proto::READY != 0)
}

/// Set budget: the form as typed into SET_BUDGET, then the account again.
pub fn set(state: &mut State) -> EventOutcome {
    use crate::wallet::{clock::now_s, meter::budget_form::budget_request};
    let asked = budget_request(&state.meter.cap_text, &state.meter.days_text, now_s());
    let body = match asked.map(|b| b.encode()) {
        Ok(Some(body)) => body,
        Ok(None) => return EventOutcome::Idle,
        Err(why) => {
            state.failure = Some(why);
            return EventOutcome::Repaint;
        }
    };
    match crate::wallet::meter::call::set_budget(&body) {
        Ok(()) => {
            state.status = b"budget set";
            state.meter.cap_text.clear();
            state.meter.days_text.clear();
        }
        Err(s) => state.failure = Some(crate::wallet::meter::said::budget_refusal(s)),
    }
    super::click::refresh(state)
}
