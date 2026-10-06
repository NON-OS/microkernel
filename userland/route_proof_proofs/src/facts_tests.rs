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

use crate::facts::{anyone_report, AnyoneBootstrap, AnyoneFacts};
use crate::report::{decode, encode, Network, Stage};
use crate::verdict::{route_verdict, RouteVerdict, Stale, ROUTE_ANYONE};

fn up() -> AnyoneFacts {
    AnyoneFacts {
        bootstrap: AnyoneBootstrap::Ready,
        usable: true,
        signatures_verified: 6,
        signatures_required: 5,
        relays: 4_900,
        valid_for_s: 7_200,
        open_circuits: 3,
        newest_path: 3,
        newest_keyed: 3,
    }
}

#[test]
fn a_transport_with_an_open_circuit_over_a_signed_directory_reads_anonymous() {
    let r = anyone_report(&up());
    assert_eq!(r.network, Network::Anyone);
    assert_eq!(r.stage, Stage::Ready);
    assert_eq!(r.valid_for_ms, 7_200_000);
    assert_eq!(decode(&encode(&r)), Some(r), "every report a transport makes is well formed");
    assert_eq!(route_verdict(Some(ROUTE_ANYONE), Some((r, 0))), RouteVerdict::Anonymous(Network::Anyone));
}

#[test]
fn a_ready_directory_with_no_open_circuit_is_still_joining() {
    let mut f = up();
    f.open_circuits = 0;
    let r = anyone_report(&f);
    assert_eq!(r.stage, Stage::Joining);
    assert_eq!((r.hops, r.hops_authenticated), (0, 0));
    assert_eq!(
        route_verdict(Some(ROUTE_ANYONE), Some((r, 0))),
        RouteVerdict::NotEstablished(Network::Anyone, Stale::Stage(Stage::Joining))
    );
}

#[test]
fn an_expired_relay_set_is_never_ready() {
    let mut f = up();
    f.usable = false;
    f.valid_for_s = 0;
    assert_eq!(anyone_report(&f).stage, Stage::Joining);
}

#[test]
fn before_the_directory_is_joined_no_directory_figure_is_claimed() {
    for b in [AnyoneBootstrap::Cold, AnyoneBootstrap::Anchored] {
        let mut f = up();
        f.bootstrap = b;
        let r = anyone_report(&f);
        assert_eq!(r.signatures_verified, 0);
        assert_eq!(r.nodes, 0);
        assert_eq!(r.valid_for_ms, 0);
        assert_ne!(r.stage, Stage::Ready);
    }
}

#[test]
fn a_circuit_with_a_hop_still_handshaking_does_not_read_authenticated() {
    let mut f = up();
    f.newest_keyed = 2;
    let r = anyone_report(&f);
    assert_eq!((r.hops, r.hops_authenticated), (3, 2));
    assert_eq!(
        route_verdict(Some(ROUTE_ANYONE), Some((r, 0))),
        RouteVerdict::NotEstablished(Network::Anyone, Stale::HopUnauthenticated)
    );
    // More keyed hops than hops can never be reported.
    f.newest_keyed = 9;
    let r = anyone_report(&f);
    assert!(r.hops_authenticated <= r.hops);
    assert!(decode(&encode(&r)).is_some());
}

#[test]
fn huge_counts_saturate_and_still_encode() {
    let mut f = up();
    f.relays = usize::MAX;
    f.open_circuits = usize::MAX;
    f.newest_path = usize::MAX;
    f.newest_keyed = usize::MAX;
    f.valid_for_s = u64::MAX;
    let r = anyone_report(&f);
    assert_eq!(r.nodes, u32::MAX);
    assert_eq!(r.routes_open, u16::MAX);
    assert_eq!(r.valid_for_ms, u64::MAX);
    assert_eq!(decode(&encode(&r)), Some(r));
}
