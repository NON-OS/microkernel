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
 * A publisher transaction one step a tick (B8), as a dispute or a refund
 * is (order/escrow_drive.rs): one settle request at a time, ASYNC, its
 * answer taken by the one who asked. With no transaction under way, the
 * screen's views are read instead. The window never waits.
 */

use nonos_libc::mk_uptime_ms;

use super::send_answer::answered;
use super::send_ask::ask;
use super::sending::Asked;
use super::views_poll::took;
use crate::wallet::settle::PUBLISHER;
use crate::wallet::state::State;

/// How often a read that repeats is asked again: GAS, the landing view,
/// or a view that did not answer.
pub const EVERY_MS: u64 = 10_000;

/// Whether the screen changed.
pub fn poll(state: &mut State) -> bool {
    let now_ms = mk_uptime_ms().max(0) as u64;
    if let Some(a) = state.publisher.asked {
        let Some(r) = state.meter.settle.take(a.op(), PUBLISHER) else { return false };
        state.publisher.asked = None;
        if let Asked::View(v) = a {
            return took(state, v, r, now_ms);
        }
        return answered(state, a, r, now_ms);
    }
    let Some((a, body)) = ask(state, now_ms) else { return false };
    if state.meter.settle.ask(a.op(), PUBLISHER, &body, now_ms).is_ok() {
        state.publisher.asked = Some(a);
    }
    false
}
