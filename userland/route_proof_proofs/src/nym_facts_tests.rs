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

//! net.nym's report from its own facts, and what the verdict makes of it:
//! anonymous only with a trusted topology in date and an authenticated
//! gateway, and never a hop claimed before both.

use crate::facts::{nym_report, NymDirectory, NymFacts, NYM_REASON_NO_CLOCK, NYM_REASON_UNTRUSTED};
use crate::report::{decode, encode, Network, Stage};
use crate::verdict::{route_verdict, RouteVerdict, Stale, ROUTE_NYM};

fn up() -> NymFacts {
    NymFacts {
        directory: NymDirectory::Ready,
        nodes: 640,
        valid_for_ms: 3_600_000,
        gateway_authenticated: true,
        route_hops: 4,
        surbs: 120,
        cover_traffic: true,
    }
}

#[test]
fn a_bound_gateway_over_a_trusted_topology_reads_anonymous() {
    let r = nym_report(&up());
    assert_eq!((r.network, r.stage), (Network::Nym, Stage::Ready));
    assert_eq!((r.signatures_verified, r.signatures_required), (1, 1));
    assert_eq!((r.hops_authenticated, r.hops, r.routes_open), (4, 4, 1));
    assert_eq!(decode(&encode(&r)), Some(r), "the report survives the wire");
    assert_eq!(route_verdict(Some(ROUTE_NYM), Some((r, 1_000))), RouteVerdict::Anonymous(Network::Nym));
}

#[test]
fn no_gateway_yet_is_joining_and_claims_no_hop() {
    let r = nym_report(&NymFacts { gateway_authenticated: false, ..up() });
    assert_eq!(r.stage, Stage::Joining);
    assert_eq!((r.hops_authenticated, r.hops, r.routes_open), (0, 0, 0));
    assert_eq!(
        route_verdict(Some(ROUTE_NYM), Some((r, 1_000))),
        RouteVerdict::NotEstablished(Network::Nym, Stale::Stage(Stage::Joining))
    );
}

#[test]
fn a_topology_short_of_ready_claims_no_directory_and_no_route() {
    for (directory, stage, reason) in [
        (NymDirectory::Missing, Stage::Cold, 0),
        (NymDirectory::NoClock, Stage::Bootstrapping, NYM_REASON_NO_CLOCK),
        (NymDirectory::Expired, Stage::Bootstrapping, 0),
        (NymDirectory::Untrusted, Stage::Failed, NYM_REASON_UNTRUSTED),
    ] {
        let r = nym_report(&NymFacts { directory, ..up() });
        assert_eq!((r.stage, r.reason), (stage, reason), "{directory:?}");
        assert_eq!((r.signatures_verified, r.nodes, r.valid_for_ms), (0, 0, 0), "{directory:?}");
        assert_eq!((r.routes_open, r.hops, r.hops_authenticated), (0, 0, 0), "{directory:?}");
        assert!(!matches!(
            route_verdict(Some(ROUTE_NYM), Some((r, 0))),
            RouteVerdict::Anonymous(_)
        ));
        assert_eq!(decode(&encode(&r)), Some(r));
    }
}

#[test]
fn a_topology_that_ran_out_since_the_report_is_not_anonymous() {
    let r = nym_report(&NymFacts { valid_for_ms: 500, ..up() });
    assert_eq!(
        route_verdict(Some(ROUTE_NYM), Some((r, 1_000))),
        RouteVerdict::NotEstablished(Network::Nym, Stale::DirectoryExpired)
    );
}

#[test]
fn counts_past_the_wire_saturate() {
    let r = nym_report(&NymFacts { nodes: usize::MAX, surbs: u32::MAX, route_hops: 300, ..up() });
    assert_eq!((r.nodes, r.surbs, r.hops), (u32::MAX, u16::MAX, u8::MAX));
    assert_eq!(decode(&encode(&r)), Some(r));
}
