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

use crate::wallet::screen::shield::sizes::sizes;
use crate::wallet::screen::shield::typed::reads_typed;
use crate::wallet::state::shield_ui::*;
use crate::wallet::state::State;

pub use super::actions_confirm::{confirm, review_self_settle, take_back};
pub use super::actions_withdraw::review_withdraw;

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
    // An escrow deposit is funded with the order's exact price (B8).
    if reads_typed(ui.screen, ui.from, SHIELD_SEND, SHIELD_REVIEW) || ui.to_deposit.is_some() {
        return ui.amount.clone();
    }
    let all = sizes(ui.asset);
    ui.size
        .and_then(|i| all.get(i as usize))
        .map(|(label, _)| label.replace(',', ""))
        .unwrap_or_default()
}

pub(super) fn early(ui: &ShieldUi) -> &'static str {
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
