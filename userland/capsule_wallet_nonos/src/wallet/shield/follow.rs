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
 * Following the spend proved last until it lands. The service looks at the
 * chain at most once a minute, republishes through the landers when one
 * dropped it, and after 30 minutes offers the owner a settlement from their
 * own account. Inputs count as spent only once it has landed.
 */

use alloc::string::String;

use super::reply;
use crate::wallet::state::State;

pub use super::follow_due::{due, sync_stale};
use super::follow_earlier::followed_earlier;

const NO_STATE: &str = "The shield service's follow did not say where the payment is. It is \
     asked again in a minute.";
/* A spend the owner settled from their own account, followed until it lands. */
pub use super::reply::SETTLING;

pub fn followed(state: &mut State, body: &str) {
    followed_earlier(state, body);
    let ui = &mut state.shield_ui;
    let Some(spend) = ui.spend.as_mut() else { return };
    let Some(r) = reply::last(body) else {
        ui.failure = Some(String::from(NO_STATE));
        return;
    };
    let settling = spend.state == SETTLING;
    spend.self_settle = r.self_settle && !settling;
    spend.minutes = r.minutes.unwrap_or(spend.minutes);
    spend.refusal = r.refusal.map(String::from);
    spend.published |= r.published;
    spend.state = String::from(reply::said(r.said, settling));
    if !reply::landed(r.said) {
        return;
    }
    spend.tx = Some(String::from(r.tx));
    ui.news = Some(reply::landed_news("The payment", r.tx));
    ui.link.last_sync_ms = 0;
}
