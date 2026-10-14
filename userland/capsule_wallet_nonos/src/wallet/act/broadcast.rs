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
use crate::wallet::net::step::Exchange;
use crate::wallet::state::State;

pub(super) use super::broadcast_finish::finish;

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
    if !state.tx.ready || state.tx.raw.is_empty() {
        state.status = b"no signed tx";
        return false;
    }
    let body = crate::wallet::rpc::request_broadcast(&state.tx.raw, 5);
    super::action::claim_network(state);
    let exchange = Exchange::begin(Route::for_wallet(), body);
    let chain = crate::wallet::chain::current().id;
    state.action = Some(Action::Broadcast {
        purpose,
        hash: state.tx.hash,
        chain,
        address: state.address,
        exchange,
    });
    state.status = b"sending";
    true
}
