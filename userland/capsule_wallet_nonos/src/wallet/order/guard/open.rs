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
 * A purchase's reading ended (B8): the consent opens, raised and focused,
 * only on terms read for the names asked; otherwise the purchase goes,
 * said in one sentence, and nothing of its terms was ever shown. Focus
 * lost over the consent starts the reading again (read_again).
 */

use alloc::boxed::Box;

use crate::wallet::order::reading_step::Terms;
use crate::wallet::order::terms::shown::{opens, unread, Unread};
use crate::wallet::state::{State, VIEW_HOME, VIEW_PURCHASE};

const LOST: &str = "The wallet's window lost focus while it asked you to buy, so it is reading \
     the terms again and will ask again.";

pub fn landed(state: &mut State, done: Result<Box<Terms>, &'static str>) {
    let Some(p) = state.meter.purchase.as_deref_mut() else { return };
    match done {
        Ok(t) => {
            (p.read, p.record) = (Ok(t.named), Some(t.record));
            if opens(p.reading.as_ref(), &p.read, p.record.as_ref()) {
                (state.view, state.scroll, state.failure) = (VIEW_PURCHASE, 0, None);
                // Refused (E_PERM under T1b) or not: unarmed until focused.
                p.guard = super::focus::raise_self();
            }
        }
        Err(why) => {
            state.meter.purchase = None;
            state.failure = Some(why);
        }
    }
}

/// Focus lost over the consent: its terms read again from the start, the
/// Shield asked again, nothing shown until the consent opens afresh.
pub fn read_again(state: &mut State) {
    let Some(p) = state.meter.purchase.as_deref_mut() else { return };
    let Unread { read, reading, record, fees, guard } = unread();
    (p.read, p.reading, p.record, p.fees, p.guard, p.armed) =
        (read, reading, record, fees, guard, false);
    (state.view, state.scroll, state.failure) = (VIEW_HOME, 0, Some(LOST));
}
