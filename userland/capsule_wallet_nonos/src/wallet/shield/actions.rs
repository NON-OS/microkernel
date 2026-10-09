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
 * What each Shield button asks of the service. Reviews are checked by the
 * service against the pool before anything is shown; nothing leaves this
 * machine until Confirm, and a payment is proved here, on this machine.
 */

use alloc::string::String;

use shield_wire::*;

use super::client::call;
use crate::wallet::screen::shield::sizes::sizes;
use crate::wallet::screen::shield::typed::reads_typed;
use crate::wallet::state::shield_ui::*;
use crate::wallet::state::State;

const NO_FRESH: &str = "A withdrawal goes only to a fresh account of this wallet, never one \
     used before, and a wallet from a private key has no other account. Restore from the \
     recovery words to withdraw.";
pub const OVERRIDE: &str = "spend early";

pub fn coin(asset: u8) -> &'static str {
    if asset == ASSET_NOX {
        "NOX"
    } else {
        "ETH"
    }
}

/* The amount in the units the service reads: the chosen standard size for
 * a deposit or a withdrawal, the typed amount for a private payment. */
pub fn amount(ui: &ShieldUi) -> String {
    if reads_typed(ui.screen, ui.from, SHIELD_SEND, SHIELD_REVIEW) {
        return ui.amount.clone();
    }
    let all = sizes(ui.asset);
    ui.size
        .and_then(|i| all.get(i as usize))
        .map(|(label, _)| label.replace(',', ""))
        .unwrap_or_default()
}

fn early(ui: &ShieldUi) -> &'static str {
    if ui.override_text == OVERRIDE {
        "1"
    } else {
        "0"
    }
}

pub fn review_deposit(state: &mut State) {
    let a = amount(&state.shield_ui);
    super::job::start(state, OP_REVIEW_SHIELD, &[coin(state.shield_ui.asset), &a]);
}

pub fn review_send(state: &mut State) {
    let a = amount(&state.shield_ui);
    super::job::start(state, OP_QUOTE, &[coin(state.shield_ui.asset), &a, "0"]);
}

pub fn review_withdraw(state: &mut State) {
    let fresh = match call(OP_FRESH_ADDRESS, &[]) {
        Ok(a) => a,
        Err(why) => {
            state.shield_ui.failure = Some(String::from(why));
            return;
        }
    };
    let Some(to) = field(&fresh.body, "address").filter(|_| fresh.status == STATUS_OK) else {
        let why = if fresh.status == STATUS_OK {
            String::from(NO_FRESH)
        } else {
            String::from(field(&fresh.body, "why").unwrap_or(super::client::NO_ANSWER))
        };
        state.shield_ui.failure = Some(why);
        return;
    };
    state.shield_ui.withdraw_to = Some(String::from(to));
    let a = amount(&state.shield_ui);
    super::job::start(state, OP_QUOTE, &[coin(state.shield_ui.asset), &a, "1"]);
}

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
            let asset = ui.asset;
            if super::job::start(state, OP_WITHDRAW, &[c, &to, &a, e]) {
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
