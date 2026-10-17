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
//! Holding a new transaction back while the last one may still land.

use super::awaiting;
use crate::wallet::act::Purpose;
use crate::wallet::chain;
use crate::wallet::state::State;

/// How long a transaction that may have gone holds the next one back: the
/// receipt is read every dozen seconds, and one not seen in this long most
/// likely never reached the node.
pub const HOLD_MS: i64 = 10 * 60 * 1000;

/// Why no new transaction may be reviewed now, or None. A broadcast whose
/// outcome is unknown may still land, and an approve not yet in a block
/// gives the stake nothing to spend: either holds the next signature back
/// until its receipt is read, or for `HOLD_MS` at most.
pub fn held_back(state: &State) -> Option<&'static str> {
    held_for(state, false)
}

/// The same for the path that signs next, the publisher screen's or a
/// payment's: the other path's sends still waiting for a block hold it, so
/// the two never take one nonce.
pub fn held_for(state: &State, publisher: bool) -> Option<&'static str> {
    let now = nonos_libc::mk_uptime_ms();
    let (chain, me) = (chain::current().id, &state.acct.address);
    if state.broadcast.sent.unknown_waiting(chain, me, now, HOLD_MS) {
        return Some(
            "The last transaction may still land. A new one waits for its receipt, \
             for ten minutes at most, so nothing is paid twice.",
        );
    }
    if state.broadcast.sent.waiting_of(chain, me, !publisher) {
        return Some("The last transaction is not in a block yet; a new one waits for it.");
    }
    if !awaiting(state) {
        return None;
    }
    let recent = now.wrapping_sub(state.broadcast.sent_at_ms) < HOLD_MS;
    if recent && state.broadcast.purpose == Purpose::Approve {
        return Some("Waiting for the approval to land in a block before the stake.");
    }
    None
}
