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

//! Network work a press starts: reading the nonce, fee and gas for a
//! payment's review, and broadcasting a signed transaction. A press only
//! begins it; the window steps it a slice a tick (`net::step`) and applies
//! the result once it finishes, its screen saying it is working meanwhile.
//!
//! One at a time: a press while an action runs does nothing, so nothing is
//! read twice over itself and nothing is sent twice. An action also holds
//! the network to itself. Both proxies keep one stream per program, so a
//! refresh under way is dropped when an action begins and none begins while
//! one runs; the refresh takes up again once it is done.

mod broadcast;
mod review;

use crate::wallet::net::step::Exchange;
use crate::wallet::send::Plan;
use crate::wallet::state::State;

pub use broadcast::{broadcast, Purpose};
pub use review::{review, review_stake};

pub enum Action {
    /// The nonce, the fee and the gas for the payment planned on the form.
    Review { plan: Plan, exchange: Exchange },
    /// The signed transaction recorded in the state, going out once.
    Broadcast {
        purpose: Purpose,
        hash: [u8; 32],
        chain: u64,
        address: [u8; 20],
        exchange: Exchange,
    },
}

/// Whether an action runs, so a press that would start another does nothing.
pub fn running(state: &State) -> bool {
    state.action.is_some()
}

/// What the running action is doing, for its screen's button.
pub fn working(state: &State) -> Option<&'static str> {
    match state.action {
        Some(Action::Review { .. }) => Some("Reading the network"),
        Some(Action::Broadcast { .. }) => Some("Sending"),
        None => None,
    }
}

/// Make way for an action: the refresh under way gives up the network.
fn claim_network(state: &mut State) {
    state.net_job = None;
}

/// One step of the running action; true when its result changed the screen.
pub fn step(state: &mut State) -> bool {
    let finished = match state.action.as_mut() {
        Some(Action::Review { exchange, .. }) | Some(Action::Broadcast { exchange, .. }) => {
            exchange.step()
        }
        None => return false,
    };
    let Some(ended) = finished else {
        return false;
    };
    match state.action.take() {
        Some(Action::Review { plan, .. }) => review::finish(state, plan, ended),
        Some(Action::Broadcast { purpose, hash, chain, address, .. }) => {
            broadcast::finish(state, purpose, hash, chain, address, ended)
        }
        None => {}
    }
    true
}
