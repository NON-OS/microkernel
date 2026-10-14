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
//! The network work a press started, and whether one is running.

use crate::wallet::net::step::Exchange;
use crate::wallet::send::Plan;
use crate::wallet::state::State;

use super::Purpose;

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
pub(super) fn claim_network(state: &mut State) {
    state.net.job = None;
}
