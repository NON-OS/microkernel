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

//! A proved send or withdrawal: its landing followed from here.

use alloc::string::String;

use nonos_libc::mk_uptime_ms;
use shield_wire::*;

use crate::wallet::state::shield_log::Kind;
use crate::wallet::state::shield_ui::*;
use crate::wallet::state::State;

use super::apply_refresh::refresh;

pub(super) fn proved(state: &mut State, op: u16, body: &str) {
    let kind = if op == OP_SEND { Kind::Send } else { Kind::Withdraw };
    let published = super::reply::taken(body);
    /* What was confirmed, not what the form holds now. */
    let (asset, amount) = state
        .shield_ui
        .proving
        .take()
        .unwrap_or_else(|| (state.shield_ui.asset, super::actions::amount(&state.shield_ui)));
    refresh(state);
    let ui = &mut state.shield_ui;
    /* The last spend may still be on its way: the service keeps it, and it
     * is followed among the earlier ones. */
    if let Some(last) = ui.spend.take().filter(|s| s.tx.is_none()) {
        ui.earlier.push(last);
    }
    ui.spend = Some(Spend {
        id: None,
        kind,
        asset,
        amount,
        published,
        state: String::from(if published {
            "handed to a lander"
        } else {
            "kept, no lander took it"
        }),
        minutes: 0,
        tx: None,
        self_settle: !published,
        refusal: field(body, "lander_refusal").map(String::from),
        weakened: fields(body, "weakened").map(String::from).collect(),
    });
    ui.link.last_follow_ms = mk_uptime_ms();
    ui.quote = None;
    ui.to.clear();
    ui.amount.clear();
    ui.override_text.clear();
    ui.screen = SHIELD_PROVING;
}
