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

use super::{Action, Purpose};
use crate::wallet::net::step::{Ended, Exchange};
use crate::wallet::rpc;
use crate::wallet::send::{Plan, STAGE_FORM, STAGE_REVIEW};
use crate::wallet::state::{State, VIEW_NOX, VIEW_SEND};

const ID_NONCE: u64 = 3;
const ID_FEE: u64 = 4;
const ID_GAS: u64 = 7;
const ID_HISTORY: u64 = 5;

const NO_ANSWER: &str = "The network did not answer. Try again.";
const SWITCHED: &str = "The network changed while this was read. Review the payment again.";

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

/// Take what the network said into the review, or into the reason there is
/// none. A form left, or a network switched, while it was read is not
/// reviewed against these readings.
pub(super) fn finish(state: &mut State, plan: Plan, ended: Ended) {
    let resp = match ended {
        Ended::Answer(resp) => resp,
        Ended::Failed(why) => return refuse(state, why),
        Ended::Unknown(why) => return refuse(state, why),
    };
    /* Read on another network: nothing of it is taken, not even the nonce. */
    if plan.chain_id != crate::wallet::chain::current().id {
        return refuse(state, SWITCHED);
    }
    let field = |id| rpc::object_for_id(&resp, id).and_then(rpc::parse_u64);
    let taken = crate::wallet::event::take_nonce_and_fee(state, field(ID_NONCE), field(ID_FEE));
    /* The fees offered come from this reading's blocks, or there are none. */
    state.offered_fees = rpc::object_for_id(&resp, ID_HISTORY)
        .and_then(crate::wallet::send::fees::parse_fee_history)
        .map(|(base, tips)| crate::wallet::send::fees::fees(base, tips));
    let purpose = plan.purpose;
    if !asked_from(state, purpose) {
        return;
    }
    if let Err(why) = taken {
        return refuse(state, core::str::from_utf8(why).unwrap_or(NO_ANSWER));
    }
    match crate::wallet::send::finish(state, plan, field(ID_GAS)) {
        Ok(d) if purpose == Purpose::Pay => {
            state.send_draft = Some(d);
            state.send_stage = STAGE_REVIEW;
            state.failure = None;
            state.scroll = 0;
        }
        Ok(d) => {
            state.stake_draft = Some(d);
            state.failure = None;
            state.scroll = 0;
        }
        Err(why) => refuse(state, why),
    }
}

/* Whether the screen the review was asked from is still up, unreviewed. */
fn asked_from(state: &State, purpose: Purpose) -> bool {
    match purpose {
        Purpose::Pay => state.view == VIEW_SEND && state.send_stage == STAGE_FORM,
        _ => state.view == VIEW_NOX && state.stake_draft.is_none(),
    }
}

/* Said on the screen it was asked from, and nowhere else. */
fn refuse(state: &mut State, why: &'static str) {
    let on_form = state.view == VIEW_SEND && state.send_stage == STAGE_FORM;
    let on_stake = state.view == VIEW_NOX && state.stake_draft.is_none();
    if on_form || on_stake {
        state.failure = Some(why);
    }
}
