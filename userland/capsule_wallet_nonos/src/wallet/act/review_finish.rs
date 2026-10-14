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

//! The network's readings taken into the payment's review, and the ids
//! each is asked under.

use super::review_refuse::{asked_from, refuse};
use super::Purpose;
use crate::wallet::net::step::Ended;
use crate::wallet::rpc;
use crate::wallet::send::{Plan, STAGE_REVIEW};
use crate::wallet::state::State;

pub(super) const ID_NONCE: u64 = 3;
pub(super) const ID_FEE: u64 = 4;
pub(super) const ID_GAS: u64 = 7;
pub(super) const ID_HISTORY: u64 = 5;

const NO_ANSWER: &str = "The network did not answer. Try again.";
const SWITCHED: &str = "The network changed while this was read. Review the payment again.";

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
    state.tx.offered_fees = rpc::object_for_id(&resp, ID_HISTORY)
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
            state.send.draft = Some(d);
            state.send.stage = STAGE_REVIEW;
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
