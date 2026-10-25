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
use crate::wallet::order::send_step::{SendStep, DISPUTE, REFUND};
use crate::wallet::state::State;

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
    let (to, size) = (send.from, *size);
    crate::wallet::shield::gas_withdraw::withdraw_gas(state, to, size)
}
