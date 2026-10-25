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
 * A publisher transaction started from a row's calls (B8): the first one
 * quoted, the rest kept to follow it. None starts while a wallet send is
 * pending (Q8), nor while the contract is not on this network. The
 * calldata is built here, once, and is what is reviewed and signed (D5).
 */

use alloc::vec::Vec;

use super::sending::{Asked, Sending};
use crate::wallet::chain;
use crate::wallet::intent::{start as first_step, IntentStep, Publisher, PublisherSend};
use crate::wallet::state::State;

pub const NOT_HERE: &str = "that contract is not on this network yet";
const NOTHING: &str = "there is nothing to send for this";
const ANSWERING: &str = "settle is still answering the last request; try again in a moment";

/// The send for `call` from this account on `chain_id`, and its step.
fn sending(
    state: &State,
    chain_id: u64,
    call: Publisher,
    queue: Vec<Publisher>,
) -> Result<Sending, &'static str> {
    let step = first_step(crate::wallet::send::held_for(state, true).is_some());
    if let IntentStep::Failed(why) = step {
        return Err(why);
    }
    let send = PublisherSend::new(&call, chain_id, state.acct.address).ok_or(NOT_HERE)?;
    Ok(Sending::new(call, send, step, queue))
}

/// A step's request still out would be taken as the new send's answer.
pub fn start(state: &mut State, calls: Vec<Publisher>) {
    let mut calls = calls.into_iter();
    let stale = state.publisher.asked.is_some_and(|a| !matches!(a, Asked::View(_)));
    let made = if stale { Err(ANSWERING) } else { calls.next().ok_or(NOTHING) };
    let made = made.and_then(|c| sending(state, chain::current().id, c, calls.collect()));
    match made {
        Ok(s) => (state.publisher.sending, state.scroll) = (Some(s), 0),
        Err(why) => state.failure = Some(why),
    }
}

/// After a call lands: the next one in its queue, on the same network,
/// quoted afresh and reviewed again.
pub fn next(state: &mut State) {
    let Some(mut s) = state.publisher.sending.take() else { return };
    if s.queue.is_empty() {
        state.publisher.sending = Some(s);
        return;
    }
    let call = s.queue.remove(0);
    match sending(state, s.send.chain_id, call, core::mem::take(&mut s.queue)) {
        Ok(n) => state.publisher.sending = Some(n),
        Err(why) => state.publisher.sending = Some(Sending { step: IntentStep::Failed(why), ..s }),
    }
}
