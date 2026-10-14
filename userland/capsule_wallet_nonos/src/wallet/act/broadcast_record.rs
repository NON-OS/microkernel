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

//! The sent transaction recorded, so its receipt is followed.

use super::broadcast::Purpose;
use crate::wallet::send::outcome::Broadcast;
use crate::wallet::state::State;

/// Record a transaction that went, or may have, for the refresh to follow;
/// forget the last one when it did not go.
pub(super) fn record(
    state: &mut State,
    purpose: Purpose,
    result: Broadcast,
    chain: u64,
    address: [u8; 20],
) {
    match result.follow() {
        Some(h) => {
            state.broadcast.sent.record(crate::wallet::send::sent::Sent {
                chain,
                from: address,
                nonce: state.tx.nonce,
                hash: h,
                at_ms: nonos_libc::mk_uptime_ms(),
                eth_cost: state.tx.cost,
                token: state.tx.token,
                fate: crate::wallet::send::sent::Fate::Waiting,
                misses: 0,
                unknown: matches!(result, Broadcast::Unknown(_)),
            });
            state.broadcast.hash = h;
            state.broadcast.purpose = purpose;
            state.broadcast.sent_at_ms = nonos_libc::mk_uptime_ms();
            state.broadcast.ready = true;
            state.broadcast.receipt_ready = false;
            state.broadcast.unknown = matches!(result, Broadcast::Unknown(_));
            /* The refresh reads the receipt first, and soon. */
            state.probe_step = 1;
        }
        None => {
            state.broadcast.ready = false;
            state.broadcast.unknown = false;
        }
    }
}
