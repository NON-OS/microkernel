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

use crate::report::{Network, Stage};
use crate::report_tests::{ready_anyone, ready_nym};
use crate::verdict::{
    route_verdict, RouteVerdict, Stale, ROUTE_ANYONE, ROUTE_DIRECT, ROUTE_NYM, STALE_AFTER_MS,
};

#[test]
fn a_fresh_ready_route_with_every_check_passed_is_anonymous() {
    assert_eq!(
        route_verdict(Some(ROUTE_ANYONE), Some((ready_anyone(), 1_000))),
        RouteVerdict::Anonymous(Network::Anyone)
    );
    assert_eq!(
        route_verdict(Some(ROUTE_NYM), Some((ready_nym(), 0))),
        RouteVerdict::Anonymous(Network::Nym)
    );
}

#[test]
fn the_direct_route_is_exposed_whatever_the_transports_say() {
    assert_eq!(route_verdict(Some(ROUTE_DIRECT), None), RouteVerdict::Exposed);
    assert_eq!(route_verdict(Some(ROUTE_DIRECT), Some((ready_nym(), 0))), RouteVerdict::Exposed);
}

#[test]
fn an_unreadable_or_unknown_route_proves_nothing() {
    assert_eq!(route_verdict(None, Some((ready_nym(), 0))), RouteVerdict::Unknown);
    for v in 3..=255u8 {
        assert_eq!(route_verdict(Some(v), Some((ready_nym(), 0))), RouteVerdict::Unknown);
    }
}

#[test]
fn a_report_about_the_other_network_does_not_count() {
    assert_eq!(
        route_verdict(Some(ROUTE_NYM), Some((ready_anyone(), 0))),
        RouteVerdict::NotEstablished(Network::Nym, Stale::NoReport)
    );
}

#[test]
fn every_single_failed_check_withholds_anonymous() {
    let ok = ready_anyone();
    let cases: [(fn(&mut crate::report::RouteReport), u64, Stale); 9] = [
        (|_| {}, STALE_AFTER_MS + 1, Stale::Old),
        (|r| r.stage = Stage::Joining, 0, Stale::Stage(Stage::Joining)),
        (|r| r.stage = Stage::Failed, 0, Stale::Stage(Stage::Failed)),
        (|r| r.signatures_verified = 4, 0, Stale::DirectoryUnsigned),
        (|r| r.signatures_required = 0, 0, Stale::DirectoryUnsigned),
        (|r| r.valid_for_ms = 500, 500, Stale::DirectoryExpired),
        (|r| r.routes_open = 0, 0, Stale::NoOpenRoute),
        (|r| r.hops = 0, 0, Stale::NoOpenRoute),
        (|r| r.hops_authenticated = 2, 0, Stale::HopUnauthenticated),
    ];
    for (i, (spoil, age, why)) in cases.iter().enumerate() {
        let mut r = ok;
        spoil(&mut r);
        assert_eq!(
            route_verdict(Some(ROUTE_ANYONE), Some((r, *age))),
            RouteVerdict::NotEstablished(Network::Anyone, *why),
            "case {i}"
        );
    }
    assert_eq!(
        route_verdict(Some(ROUTE_ANYONE), None),
        RouteVerdict::NotEstablished(Network::Anyone, Stale::NoReport)
    );
}

#[test]
fn anonymous_is_never_concluded_unless_every_condition_holds() {
    // Exhaustive over the fields that decide, at the edges of each.
    let mut s = 0xDEAD_BEEF_CAFE_F00Du64;
    for _ in 0..100_000 {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        let mut r = ready_nym();
        r.stage = [Stage::Cold, Stage::Bootstrapping, Stage::Joining, Stage::Ready, Stage::Failed]
            [(s % 5) as usize];
        r.signatures_verified = (s >> 3) as u8 % 4;
        r.signatures_required = (s >> 5) as u8 % 4;
        r.valid_for_ms = (s >> 7) % 60_000;
        r.routes_open = ((s >> 23) % 3) as u16;
        r.hops = ((s >> 25) % 5) as u8;
        r.hops_authenticated = r.hops.min(((s >> 28) % 5) as u8);
        let age = (s >> 31) % 40_000;
        let anonymous = route_verdict(Some(ROUTE_NYM), Some((r, age)))
            == RouteVerdict::Anonymous(Network::Nym);
        let should = r.stage == Stage::Ready
            && age <= STALE_AFTER_MS
            && r.signatures_required > 0
            && r.signatures_verified >= r.signatures_required
            && r.valid_for_ms > age
            && r.routes_open > 0
            && r.hops > 0
            && r.hops_authenticated == r.hops;
        assert_eq!(anonymous, should, "{r:?} age {age}");
    }
}
