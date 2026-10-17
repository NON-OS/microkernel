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
 * Settle's answer to the publisher screen's request (send_ask.rs), taken
 * by the step that asked: the quote, the balance and shared nonce (Q8),
 * GAS again at Sign, unsure or dropped (send_retry.rs), the broadcast's
 * answer, kept apart from the hash followed, the call's view until it
 * lands, unless the ledger says it will not. Any other answer is dropped.
 */

use alloc::vec::Vec;

use nonos_nox_config::address;
use nonos_settle_proto::{parse_read_reply, Gas, Quote};

use super::send_drive::EVERY_MS;
use super::sending::Asked;
use super::views::Views;
use crate::wallet::intent::{after_gas, after_quote, landed, may_land, IntentStep};
use crate::wallet::settle::broadcast::sent;
use crate::wallet::state::State;

const NO_QUOTE: &str = "settle did not quote the transaction; try again";

/// Whether the screen changed. The screen's views are views_poll.rs's.
pub fn answered(state: &mut State, a: Asked, r: Result<Vec<u8>, i32>, now_ms: u64) -> bool {
    let words = |r: Result<Vec<u8>, i32>| r.ok().and_then(|b| parse_read_reply(&b));
    let gas = |r: Result<Vec<u8>, i32>| r.ok().and_then(|b| Gas::parse(&b));
    let Some(s) = state.publisher.sending.as_mut() else { return false };
    if let (Asked::Gas, true) = (a, s.pressed) {
        return super::send_sign::nonce_read(state, gas(r));
    }
    if let (Asked::Gas, IntentStep::Unsure { .. } | IntentStep::Sent(_)) = (a, s.step) {
        return super::send_retry::reread(state, gas(r), now_ms);
    }
    let chain_id = s.send.chain_id;
    let at = |c| address(chain_id, c).ok();
    s.step = match (a, s.step) {
        (Asked::Intent, IntentStep::Quoting) => match r.ok().and_then(|b| Quote::parse(&b)) {
            Some(q) => after_quote(&s.send, &q, &at),
            None => IntentStep::Failed(NO_QUOTE),
        },
        (Asked::Gas, step @ IntentStep::Funds(_)) => {
            s.next_ms = now_ms + EVERY_MS;
            gas(r).map_or(step, |g| after_gas(&state.broadcast.sent, &s.send, step, &g))
        }
        (Asked::Broadcast, IntentStep::Broadcasting) => super::send_retry::sent(s, sent(r), now_ms),
        (Asked::Landed, step) if may_land(&state.broadcast.sent, &step) => match words(r) {
            Some(w) if landed(&s.call, &w) => IntentStep::Landed,
            _ => step,
        },
        (_, step) => step,
    };
    if s.step == IntentStep::Landed {
        state.publisher.views = Views::default();
        super::send_start::next(state);
    }
    true
}
