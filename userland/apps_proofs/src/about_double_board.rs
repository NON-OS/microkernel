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

//! The board as sampled: a Nym route up and fresh, its directory checked, and
//! three capsules that hold Network.

use alloc::vec;

use nonos_route_proof::{Network, RouteReport, RouteVerdict, Stage, ROUTE_NYM};

use super::census::Census;
use super::session::{Boot, Mark};
use super::{Holder, Snapshot};

fn report() -> RouteReport {
    RouteReport {
        network: Network::Nym,
        stage: Stage::Ready,
        signatures_verified: 2,
        signatures_required: 2,
        nodes: 240,
        valid_for_ms: 3_600_000,
        routes_open: 3,
        hops_authenticated: 3,
        hops: 3,
        surbs: 10,
        cover_traffic: true,
        reason: 0,
    }
}

pub fn read() -> Snapshot {
    let name = *b"browser\0\0\0\0\0\0\0\0\0";
    let h = Holder { pid: 31, name, name_len: 7, measurement: [9; 32], authority: 0 };
    let (running, admitted, vendor, sandboxed) = (50, 44, 40, 5);
    let census = Some(Census { running, admitted, vendor, sandboxed, ..Default::default() });
    let boot = Boot { signature: Mark::Holds, attestation: Mark::Holds, proof: Mark::Unknown };
    let (holders, chosen, board) =
        (vec![h; 3], Some(ROUTE_NYM), Some((Some((report(), 1500)), None)));
    let route = RouteVerdict::Anonymous(Network::Nym);
    Snapshot { boot, census, holders, chosen, board, route }
}
