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
 * by the step that asked it: the quote held to the calldata's own decode,
 * the balance against the most it can cost, the pending nonce at Sign
 * (Q8), the broadcast's hash, the call's view until it lands. An answer
 * the step no longer waits for is dropped.
 */

use alloc::vec::Vec;

use nonos_nox_config::address;
use nonos_settle_proto::{parse_read_reply, Gas, Quote};

use super::send_drive::EVERY_MS;
use super::sending::Asked;
use super::views::Views;
use crate::wallet::intent::{after_balance, after_quote, landed, IntentStep};
use crate::wallet::settle::broadcast::sent;
use crate::wallet::state::State;

const NO_QUOTE: &str = "settle did not quote the transaction; try again";
const NOT_SENT: &str = "settle did not send the transaction; it may not have gone";

/// Whether the screen changed. The screen's views are views_poll.rs's.
pub fn answered(state: &mut State, a: Asked, r: Result<Vec<u8>, i32>, now_ms: u64) -> bool {
    let words = |r: Result<Vec<u8>, i32>| r.ok().and_then(|b| parse_read_reply(&b));
    let Some(s) = state.publisher.sending.as_mut() else { return false };
    if let (Asked::Gas, true) = (a, s.pressed) {
        return super::send_sign::nonce_read(state, r.ok().and_then(|b| Gas::parse(&b)));
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
            let g = r.ok().and_then(|b| Gas::parse(&b));
            g.map_or(step, |g| after_balance(step, g.balance))
        }
        (Asked::Broadcast, IntentStep::Broadcasting) => {
            s.next_ms = now_ms;
            sent(r).map_or(IntentStep::Failed(NOT_SENT), IntentStep::Sent)
        }
        (Asked::Landed, step) if words(r).is_some_and(|w| landed(&s.call, &w)) => {
            matches!(step, IntentStep::Sent(_)).then_some(IntentStep::Landed).unwrap_or(step)
        }
        (_, step) => step,
    };
    if s.step == IntentStep::Landed {
        state.publisher.views = Views::default();
        super::send_start::next(state);
    }
    true
}
