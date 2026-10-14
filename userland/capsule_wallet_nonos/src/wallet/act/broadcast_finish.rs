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

//! What a broadcast came to, taken into the state once it ends.

use super::broadcast::Purpose;
use super::broadcast_record::record;
use super::broadcast_show::{said, show};
use crate::wallet::net::step::Ended;
use crate::wallet::send::outcome::{judge, Carried};
use crate::wallet::state::State;

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
    record(state, purpose, result, chain, address);
    show(state, purpose, result);
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
