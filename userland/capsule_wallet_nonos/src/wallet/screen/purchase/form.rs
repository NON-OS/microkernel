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

//! A voucher channel's form on the purchase screen: the deposit, the days
//! it runs and the listing's cap; digits, Tab, Backspace, Enter to open.

use nonos_app_skeleton::{EventOutcome, PaintBuffer, KEY_BACKSPACE, KEY_ENTER, KEY_TAB};

use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::GAP;
use crate::wallet::meter::budget_form::push_digit;
use crate::wallet::order::intake::{Pending, Step};
use crate::wallet::order::read::Named;
use crate::wallet::order::voucher_form::{FIELD_CAP, FIELD_DAYS, FIELD_DEPOSIT};
use crate::wallet::screen::hits::{self, Press};
use crate::wallet::screen::shield::parts::field;
use crate::wallet::state::{State, VIEW_PURCHASE};

/// Whether the purchase is a voucher channel still to be opened.
pub fn open_form(p: &Pending) -> bool {
    matches!((&p.step, &p.read), (Step::Consent, Ok(Named::Voucher(_)))) && p.held.offers_form()
}

pub fn paint(p: &Pending, fb: &mut PaintBuffer, c: Rect, y: u32) -> u32 {
    let f = &p.form;
    let mut h = 0;
    for (id, name, text, hint) in [
        (FIELD_DEPOSIT, "DEPOSIT, WHOLE NOX", &f.deposit, "at least one use"),
        (FIELD_DAYS, "FOR HOW MANY DAYS", &f.days, "1 to 365"),
        (FIELD_CAP, "THIS LISTING'S CAP, WHOLE NOX", &f.cap, "the deposit"),
    ] {
        let (at, used) = field(fb, c, y + h, name, text, hint, f.focus == id);
        hits::put(Press::Field(id), at);
        h += used + GAP / 2;
    }
    h
}

pub fn key(state: &mut State, code: u32) -> Option<EventOutcome> {
    if state.view != VIEW_PURCHASE {
        return None;
    }
    let p = state.meter.purchase.as_deref_mut().filter(|p| open_form(p))?;
    let f = &mut p.form;
    let (text, max) = match f.focus {
        FIELD_DEPOSIT => (&mut f.deposit, 13),
        FIELD_CAP => (&mut f.cap, 13),
        _ => (&mut f.days, 3),
    };
    match code {
        KEY_TAB => f.focus = (f.focus + 1) % 3,
        KEY_ENTER => return Some(super::click::open_channel(state)),
        KEY_BACKSPACE => {
            text.pop();
        }
        c if push_digit(text, c, max) => {}
        _ => return None,
    }
    state.failure = None;
    Some(EventOutcome::Repaint)
}
