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

//! A press on a made order's dispute or refund (B8, v1.4.51): start one,
//! start it again after a failure, or pay its gas from the Shield, a
//! standard ETH amount withdrawn privately to the refund address.

use nonos_app_skeleton::EventOutcome;

use super::escrow_words::offered;
use crate::wallet::order::escrow_offer::Offer;
use crate::wallet::order::escrowing::Escrowing;
use crate::wallet::order::fees::exact;
use crate::wallet::order::send_step::{SendStep, DISPUTE, REFUND};
use crate::wallet::shield::{actions, open};
use crate::wallet::state::shield_ui::SHIELD_WITHDRAW;
use crate::wallet::state::{State, VIEW_SHIELD};

const OFF_SHIELD: &str = "The Shield is on another network: switch to Sepolia to pay the gas.";
const ETH: u8 = 0;

/// Whether the made order offers an action beside Close.
pub fn offering(state: &State) -> bool {
    state.meter.purchase.as_deref().and_then(offered).is_some()
}

pub fn click(state: &mut State) -> EventOutcome {
    let Some(p) = state.meter.purchase.as_deref_mut() else { return EventOutcome::Idle };
    let kind = p.escrow.as_ref().map_or(REFUND, |e| e.kind);
    match offered(p) {
        Some(Offer::Dispute) => p.escrow = Some(Escrowing::new(DISPUTE)),
        Some(Offer::Refund) => p.escrow = Some(Escrowing::new(REFUND)),
        Some(Offer::Again) => p.escrow.get_or_insert_with(|| Escrowing::new(kind)).again(),
        Some(Offer::PayGas) => return pay_gas(state),
        Some(Offer::GasWaiting) | None => return EventOutcome::Idle,
    }
    EventOutcome::Repaint
}

/// The Shield's withdraw of the gas's size to the refund address.
fn pay_gas(state: &mut State) -> EventOutcome {
    let e = state.meter.purchase.as_deref().and_then(|p| p.escrow.as_ref());
    let Some(SendStep::Gas { send, size, .. }) = e.map(|e| &e.step) else {
        return EventOutcome::Idle;
    };
    let (to, amount) = (send.from, exact(*size));
    if !open::here() {
        state.failure = Some(OFF_SHIELD);
        return EventOutcome::Repaint;
    }
    let ui = &mut state.shield_ui;
    (ui.screen, ui.asset, ui.size, ui.amount) = (SHIELD_WITHDRAW, ETH, None, amount);
    (ui.to_meter, ui.to_deposit, ui.failure) = (false, Some(to), None);
    state.view = VIEW_SHIELD;
    state.scroll = 0;
    actions::review_withdraw(state);
    EventOutcome::Repaint
}
