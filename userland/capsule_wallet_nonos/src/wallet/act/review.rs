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

//! A payment's review, read from the network without stopping the window.
//! The form's checks run at the press; the nonce, the fee, the node's gas
//! estimate and the recent blocks' fees then come in one batched request on
//! one connection. The review shows only once
//! all three are in and the payment still adds up. A staking transaction
//! is reviewed the same way, on the stake screen.

use alloc::vec::Vec;

use nonos_route_link::Route;

use super::Action;
use crate::wallet::net::step::Exchange;
use crate::wallet::rpc;
use crate::wallet::send::Plan;
use crate::wallet::state::State;

pub(super) use super::review_finish::finish;
use super::review_finish::{ID_FEE, ID_GAS, ID_HISTORY, ID_NONCE};

/// Begin reading the network for the payment on the form, or say why it
/// cannot be made. Does nothing while an action runs.
pub fn review(state: &mut State) -> Result<(), &'static str> {
    if super::running(state) {
        return Ok(());
    }
    let plan = crate::wallet::send::plan(state)?;
    begin(state, plan)
}

/// The same for the staking transaction the stake screen asks for next.
pub fn review_stake(state: &mut State) -> Result<(), &'static str> {
    if super::running(state) {
        return Ok(());
    }
    let plan = crate::wallet::send::stake::plan(state)?;
    begin(state, plan)
}

fn begin(state: &mut State, plan: Plan) -> Result<(), &'static str> {
    if let Some(why) = crate::wallet::send::held_back(state) {
        return Err(why);
    }
    let gas = rpc::request_estimate_gas(&state.address, &plan.to, plan.value, &plan.data, ID_GAS);
    let parts: [Vec<u8>; 4] = [
        rpc::request_nonce(&state.address, ID_NONCE),
        rpc::request_fee(ID_FEE),
        gas,
        rpc::request_fee_history(ID_HISTORY),
    ];
    let refs: Vec<&[u8]> = parts.iter().map(Vec::as_slice).collect();
    let body = rpc::request_batch(&refs);
    super::claim_network(state);
    let exchange = Exchange::begin(Route::for_wallet(), body);
    state.action = Some(Action::Review { plan, exchange });
    state.failure = None;
    Ok(())
}
