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
 * Confirm on a Shield review, and the self-settle and take-back presses:
 * nothing leaves this machine until Confirm, and a payment is proved here.
 */

use alloc::string::String;

use shield_wire::*;

use crate::wallet::state::shield_ui::*;
use crate::wallet::state::State;

use super::actions::{amount, coin, early};

/* Confirm on the review: send what was reviewed, or prove the payment. */
pub fn confirm(state: &mut State) {
    let ui = &state.shield_ui;
    let (c, a, e) = (coin(ui.asset), amount(ui), early(ui));
    match ui.from {
        SHIELD_DEPOSIT | SHIELD_SETTLE => {
            let Some(id) = ui.review.as_ref().map(|r| r.id.clone()) else { return };
            super::job::start(state, OP_CONFIRM, &[&id]);
        }
        SHIELD_SEND => {
            let to = ui.to.clone();
            let asset = ui.asset;
            if super::job::start(state, OP_SEND, &[c, &to, &a, e]) {
                state.shield_ui.proving = Some((asset, a));
                state.shield_ui.screen = SHIELD_PROVING;
            }
        }
        _ => {
            let Some(to) = ui.withdraw_to.clone() else { return };
            let (asset, deposit) = (ui.asset, ui.to_deposit);
            if super::job::start(state, OP_WITHDRAW, &[c, &to, &a, e]) {
                crate::wallet::order::gas_paid::started(state, asset, deposit);
                state.shield_ui.proving = Some((asset, a));
                state.shield_ui.screen = SHIELD_PROVING;
            }
        }
    }
}

/* Settle from the account the last spend, or the earlier one kept as `id`. */
pub fn review_self_settle(state: &mut State, id: Option<String>) {
    let fields: alloc::vec::Vec<&str> = id.as_deref().into_iter().collect();
    if super::job::start(state, OP_REVIEW_SELF_SETTLE, &fields) {
        state.shield_ui.settling_id = id;
    }
}

pub fn take_back(state: &mut State) {
    super::job::start(state, OP_TAKE_BACK, &[]);
}
