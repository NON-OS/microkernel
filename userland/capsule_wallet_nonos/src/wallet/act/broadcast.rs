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

//! A signed transaction sent to the node, once, without stopping the
//! window. It goes over a connection that has passed every check the data
//! path holds, and is never sent again by this wallet: an
//! exchange that broke once the request had begun to go is said as unknown
//! and its receipt followed (`send::outcome`). The receipt is read by the
//! refresh, which this asks for at once.

use nonos_route_link::Route;

use super::Action;
use crate::wallet::net::step::{Ended, Exchange};
use crate::wallet::send::outcome::{judge, Broadcast, Carried};
use crate::wallet::send::STAGE_SENT;
use crate::wallet::state::{State, VIEW_NOX, VIEW_SEND};

/// What the transaction was, for what is said once it went.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Purpose {
    Pay,
    Approve,
    Stake,
    Unstake,
}

/// Begin sending the transaction `record_tx` last recorded. False, with the
/// status saying why, when there is none; false and nothing said while
/// another action runs.
pub fn broadcast(state: &mut State, purpose: Purpose) -> bool {
    if super::running(state) {
        return false;
    }
    if !state.tx_ready || state.tx_raw.is_empty() {
        state.status = b"no signed tx";
        return false;
    }
    let body = crate::wallet::rpc::request_broadcast(&state.tx_raw, 5);
    super::claim_network(state);
    let exchange = Exchange::begin(Route::for_wallet(), body);
    let chain = crate::wallet::chain::current().id;
    state.action = Some(Action::Broadcast {
        purpose,
        hash: state.tx_hash,
        chain,
        address: state.address,
        exchange,
    });
    state.status = b"sending";
    true
}

/// Take what the broadcast came to.
pub(super) fn finish(
    state: &mut State,
    purpose: Purpose,
    hash: [u8; 32],
    chain: u64,
    address: [u8; 20],
    ended: Ended,
) {
    let carried = match ended {
        Ended::Answer(resp) => carried(&resp),
        Ended::Failed(why) => Carried::NotSent(why),
        Ended::Unknown(_) => Carried::Unknown,
    };
    let result = judge(carried, &hash);
    state.status = said(purpose, result);
    /* Sent from an account or a network no longer open: said, and its
     * receipt is not followed against the one open now. */
    if address != state.address || chain != crate::wallet::chain::current().id {
        return;
    }
    match result.follow() {
        Some(h) => {
            state.sent.record(crate::wallet::send::sent::Sent {
                chain,
                from: address,
                nonce: state.tx_nonce,
                hash: h,
                at_ms: nonos_libc::mk_uptime_ms(),
                eth_cost: state.tx_cost,
                token: state.tx_token,
                fate: crate::wallet::send::sent::Fate::Waiting,
                misses: 0,
                unknown: matches!(result, Broadcast::Unknown(_)),
            });
            state.broadcast_hash = h;
            state.follow_purpose = purpose;
            state.sent_at_ms = nonos_libc::mk_uptime_ms();
            state.broadcast_ready = true;
            state.receipt_ready = false;
            state.broadcast_unknown = matches!(result, Broadcast::Unknown(_));
            /* The refresh reads the receipt first, and soon. */
            state.probe_step = 1;
        }
        None => {
            state.broadcast_ready = false;
            state.broadcast_unknown = false;
        }
    }
    let went = result.follow().is_some();
    match purpose {
        Purpose::Pay => {
            /* Always the Sent screen: it says what came of this payment,
             * and nothing on it signs again. */
            state.send_stage = STAGE_SENT;
            state.send_draft = None;
            if went {
                state.send_amount = crate::wallet::num::Amount::new();
            }
            if state.view == VIEW_SEND {
                state.failure = refusal(result);
                state.scroll = 0;
            }
        }
        Purpose::Approve | Purpose::Stake | Purpose::Unstake => {
            /* The step moves on when the receipt says so (`send::follow`). */
            state.stake_draft = None;
            if state.view == VIEW_NOX {
                state.failure = if went { None } else { core::str::from_utf8(state.status).ok() };
            }
        }
    }
}

/// What the node's answer to the broadcast carried: the hash it took the
/// transaction under, or its refusal in words. A node that already holds
/// this very transaction has it on its way, so it is followed as sent.
fn carried(resp: &[u8]) -> Carried {
    use crate::wallet::rpc::{broadcast_error, parse_hash32, NodeSaid};
    if let Some(hash) = parse_hash32(resp) {
        return Carried::Answer(Some(hash));
    }
    match broadcast_error(resp) {
        NodeSaid::AlreadyKnown => Carried::Answer(Some([0u8; 32])),
        NodeSaid::Refused(why) => Carried::Refused(why),
    }
}

/// The reason the screen shows when the transaction did not go.
fn refusal(result: Broadcast) -> Option<&'static str> {
    match result {
        Broadcast::Sent(_) | Broadcast::Unknown(_) => None,
        Broadcast::Refused(why) | Broadcast::NotSent(why) => Some(why),
    }
}

/// The status line once the broadcast is over.
fn said(purpose: Purpose, result: Broadcast) -> &'static [u8] {
    match (purpose, result) {
        (_, Broadcast::Refused(why)) | (_, Broadcast::NotSent(why)) => why.as_bytes(),
        (Purpose::Pay, Broadcast::Sent(_)) => b"payment sent",
        (Purpose::Pay, Broadcast::Unknown(_)) => b"payment may be sent, following its receipt",
        (Purpose::Approve, Broadcast::Sent(_)) => b"approval sent, waiting for its receipt",
        (Purpose::Approve, Broadcast::Unknown(_)) => b"approval may be sent, following its receipt",
        (Purpose::Stake, Broadcast::Sent(_)) => b"stake sent",
        (Purpose::Stake, Broadcast::Unknown(_)) => b"stake may be sent, following its receipt",
        (Purpose::Unstake, Broadcast::Sent(_)) => b"unstake sent",
        (Purpose::Unstake, Broadcast::Unknown(_)) => b"unstake may be sent, following its receipt",
    }
}
