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

//! Fund a made order from the Shield (B8, the plan): the withdraw set to NOX,
//! the order's exact price, which the Shield quoted as standard at consent,
//! and its deposit address, asked for the pool's quote at once; its review
//! shows the fees before the person confirms there.

use nonos_app_skeleton::EventOutcome;

use crate::wallet::order::fund_ask::fund_ask;
use crate::wallet::order::intake::Step;
use crate::wallet::shield::{actions, open};
use crate::wallet::state::shield_ui::SHIELD_WITHDRAW;
use crate::wallet::state::{State, VIEW_SHIELD};

const OFF_SHIELD: &str = "The Shield is on another network: switch to Sepolia to fund the order.";

/// Whether the purchase is at its funding step.
pub fn funding(state: &State) -> bool {
    state.meter.purchase.as_deref().is_some_and(|p| matches!(p.step, Step::Fund(_)))
}

pub fn fund(state: &mut State) -> EventOutcome {
    let Some(p) = state.meter.purchase.as_deref() else { return EventOutcome::Idle };
    // Only a price the Shield quoted is funded, at exactly that amount.
    let Some(ask) = fund_ask(&p.step, &p.read, &p.fees) else { return EventOutcome::Idle };
    if !open::here() {
        state.failure = Some(OFF_SHIELD);
        return EventOutcome::Repaint;
    }
    let ui = &mut state.shield_ui;
    (ui.screen, ui.asset, ui.size, ui.amount) = (SHIELD_WITHDRAW, ask.asset, None, ask.amount);
    (ui.payee.to_meter, ui.payee.to_deposit, ui.failure) = (false, Some(ask.to), None);
    state.view = VIEW_SHIELD;
    state.scroll = 0;
    actions::review_withdraw(state);
    EventOutcome::Repaint
}
