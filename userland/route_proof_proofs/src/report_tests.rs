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

use crate::authorize::{may_report, CAP_NETWORK};
use crate::report::{decode, encode, Network, RouteReport, Stage, REPORT_LEN, REPORT_VERSION};

pub(crate) fn ready_anyone() -> RouteReport {
    RouteReport {
        network: Network::Anyone,
        stage: Stage::Ready,
        signatures_verified: 5,
        signatures_required: 5,
        nodes: 4_812,
        valid_for_ms: 3 * 3_600_000,
        routes_open: 2,
        hops_authenticated: 3,
        hops: 3,
        surbs: 0,
        cover_traffic: false,
        reason: 0,
    }
}

pub(crate) fn ready_nym() -> RouteReport {
    RouteReport {
        network: Network::Nym,
        stage: Stage::Ready,
        signatures_verified: 1,
        signatures_required: 1,
        nodes: 240,
        valid_for_ms: 3_600_000,
        routes_open: 1,
        hops_authenticated: 4,
        hops: 4,
        surbs: 40,
        cover_traffic: true,
        reason: 0,
    }
}

fn xorshift(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

#[test]
fn a_report_survives_the_wire_unchanged() {
    for r in [ready_anyone(), ready_nym()] {
        let wire = encode(&r);
        assert_eq!(wire.len(), REPORT_LEN);
        assert_eq!(wire[0], REPORT_VERSION);
        assert_eq!(decode(&wire), Some(r));
    }
}

#[test]
fn every_wrong_length_is_refused() {
    let wire = encode(&ready_anyone());
    for n in 0..wire.len() {
        assert_eq!(decode(&wire[..n]), None, "{n} bytes accepted");
    }
    let mut long = [0u8; REPORT_LEN + 1];
    long[..REPORT_LEN].copy_from_slice(&wire);
    assert_eq!(decode(&long), None);
}

#[test]
fn unknown_versions_networks_stages_and_flags_are_refused() {
    let wire = encode(&ready_nym());
    let mut w = wire;
    w[0] = REPORT_VERSION + 1;
    assert_eq!(decode(&w), None);
    for bad in [0u8, 3, 255] {
        let mut w = wire;
        w[1] = bad;
        assert_eq!(decode(&w), None, "network {bad}");
    }
    for bad in [5u8, 9, 255] {
        let mut w = wire;
        w[2] = bad;
        assert_eq!(decode(&w), None, "stage {bad}");
    }
    let mut w = wire;
    w[7] = 2;
    assert_eq!(decode(&w), None, "cover flag 2");
}

#[test]
fn more_authenticated_hops_than_hops_is_refused() {
    let mut r = ready_anyone();
    r.hops_authenticated = 4;
    assert_eq!(decode(&encode(&r)), None);
}

#[test]
fn a_reserved_byte_set_is_refused() {
    let wire = encode(&ready_anyone());
    for i in 25..REPORT_LEN {
        let mut w = wire;
        w[i] = 1;
        assert_eq!(decode(&w), None, "reserved byte {i}");
    }
}

#[test]
fn random_bytes_never_panic_and_what_decodes_reencodes_identically() {
    let mut s = 0x1234_5678_9ABC_DEF1u64;
    let mut decoded = 0u32;
    for _ in 0..200_000 {
        let mut b = [0u8; REPORT_LEN];
        for x in b.iter_mut() {
            *x = xorshift(&mut s) as u8;
        }
        // Steer a share of the inputs toward the valid shape, so the
        // round-trip half of the property is exercised, not just refusal.
        if xorshift(&mut s) % 2 == 0 {
            b[0] = REPORT_VERSION;
            b[1] = 1 + (b[1] % 2);
            b[2] %= 5;
            b[7] %= 2;
            b[5] = b[6].min(b[5]);
            for x in b[25..].iter_mut() {
                *x = 0;
            }
        }
        if let Some(r) = decode(&b) {
            decoded += 1;
            assert_eq!(encode(&r), b);
        }
    }
    assert!(decoded > 10_000, "only {decoded} decoded");
}

#[test]
fn only_each_networks_own_transport_holding_network_may_report() {
    assert!(may_report(b"net.anon", CAP_NETWORK, Network::Anyone));
    assert!(may_report(b"net.nym", CAP_NETWORK | 0x18, Network::Nym));
    // Right name, no Network: it carries nothing, so it may say nothing.
    assert!(!may_report(b"net.anon", 0x18, Network::Anyone));
    // The other network's transport.
    assert!(!may_report(b"net.nym", CAP_NETWORK, Network::Anyone));
    assert!(!may_report(b"net.anon", CAP_NETWORK, Network::Nym));
    // Anything else holding Network: the browser, a socket service, a guest.
    for name in [&b"app.browser"[..], b"net.socks5", b"net.tcp", b"linux", b"net.anon ", b"", b"net.ano"] {
        assert!(!may_report(name, u64::MAX, Network::Anyone));
        assert!(!may_report(name, u64::MAX, Network::Nym));
    }
}
