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

//! The facts this transport checked itself, for the board's report.

use nonos_route_proof::{anyone_report, AnyoneBootstrap, AnyoneFacts, RouteReport};

use crate::circuit::CircuitStage;
use crate::directory::authority::REQUIRED_SIGNATURES;
use crate::manager::{Bootstrap, Manager};

/// The report for `state` at `now` (the manager's whole-second clock).
///
/// The signature count is the quorum check's own (`dir_quorum::signed`), and a
/// hop counts as keyed once its ntor handshake completed, which is what adds it
/// to the circuit's hops. No relay is named: the board is readable by any local
/// caller, and which guard this machine uses is not theirs to learn.
pub fn build(state: &Manager, now: u64) -> RouteReport {
    let open = || state.circuits.iter().filter(|c| c.stage == CircuitStage::Open);
    let newest = open().max_by_key(|c| c.id);
    anyone_report(&AnyoneFacts {
        bootstrap: match state.bootstrap {
            // A refresh serves from the consensus in hand (refresh_rule.rs).
            _ if state.refreshing => AnyoneBootstrap::Ready,
            Bootstrap::Cold => AnyoneBootstrap::Cold,
            Bootstrap::Anchored => AnyoneBootstrap::Anchored,
            Bootstrap::Joining => AnyoneBootstrap::Joining,
            Bootstrap::Ready => AnyoneBootstrap::Ready,
        },
        usable: state.usable_at(now),
        signatures_verified: state.consensus_signatures,
        signatures_required: u8::try_from(REQUIRED_SIGNATURES).unwrap_or(u8::MAX),
        relays: state.relays.len(),
        valid_for_s: state.valid_until.saturating_sub(now),
        open_circuits: open().count(),
        newest_path: newest.map_or(0, |c| c.path.len()),
        newest_keyed: newest.map_or(0, |c| c.hops.len()),
    })
}
