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
 * The settle request a publisher transaction's step needs (send_drive.rs):
 * INTENT for the quote with the wallet's own calldata, GAS for the balance
 * and, once Sign is pressed, a broadcast is unsure or the ledger says the
 * send was dropped, for the pending nonce; BROADCAST of what the keyring
 * signed; READ of the call's own view until it lands, while it may.
 * With none under way, the next of the screen's views (views_poll.rs).
 */

use alloc::vec::Vec;

use nonos_nox_config::address;
use nonos_settle_proto::{Gas, Intent, Read};

use super::kec::Kec;
use super::send_drive::EVERY_MS;
use super::sending::Asked;
use crate::wallet::intent::{dropped, landed_read, may_land, IntentStep};
use crate::wallet::settle::broadcast::body;
use crate::wallet::state::State;

/// The request to send now and its body, or None.
pub fn ask(state: &mut State, now_ms: u64) -> Option<(Asked, Vec<u8>)> {
    let Some(s) = state.publisher.sending.as_mut() else {
        return super::views_poll::ask(state, now_ms);
    };
    let (chain_id, from) = (s.send.chain_id, s.send.from);
    let due = now_ms >= s.next_ms;
    let gas = Some((Asked::Gas, Gas::request(chain_id, &from).to_vec()));
    match s.step {
        IntentStep::Quoting => {
            let (kind, calldata) = (s.send.kind, s.send.calldata.clone());
            let i = Intent { chain_id, kind, from, value: 0, calldata };
            let Some(b) = i.encode() else {
                s.step = IntentStep::Failed("the transaction is too long for settle to quote");
                return None;
            };
            Some((Asked::Intent, b))
        }
        IntentStep::Funds(_) if due => gas,
        IntentStep::Review { .. } if s.pressed => gas,
        IntentStep::Unsure { .. } if due => gas,
        IntentStep::Broadcasting => Some((Asked::Broadcast, body(chain_id, s.raw.as_deref()?))),
        IntentStep::Sent(_) if due && dropped(&state.broadcast.sent, &s.step) => gas,
        IntentStep::Sent(_) if due && may_land(&state.broadcast.sent, &s.step) => {
            s.next_ms = now_ms + EVERY_MS;
            let at = |c| address(chain_id, c).ok();
            let (view, args) = landed_read(&Kec, &s.call, &from, &at)?;
            let r = Read { chain_id, view, args };
            Some((Asked::Landed, r.encode()?))
        }
        _ => None,
    }
}
