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

//! A DRHD may name as many device scopes as its u8 count holds, 255: one root
//! bus alone has 256 functions before any SR-IOV VF. Past that the unit is
//! truncated, and a device it may name is refused (fail closed).

use super::dmar_scope_count::pci_scopes_named;
use super::dmar_scope_walk::walk_scopes;
use super::scope::{scope_covers, translated_by_some, Cover, Scope, MAX_SCOPES};
use super::tests::{bridges, drhd, parse, scope};

/// `n` endpoint scopes on bus 0: device i % 32, function i / 32.
fn endpoints(n: usize) -> Vec<u8> {
    let s: Vec<Vec<u8>> =
        (0..n).map(|i| scope(1, 0, &[((i % 32) as u8, (i / 32) as u8)])).collect();
    drhd(0, 0, &s)
}

fn nth(i: usize) -> (u8, u8, u8) {
    (0, (i % 32) as u8, (i / 32) as u8)
}

#[test]
fn eighteen_endpoint_scopes_cover_the_seventeenth_and_eighteenth() {
    let b = endpoints(18);
    let u = parse(&b);
    assert!(!u.truncated);
    assert_eq!((u.count, pci_scopes_named(&b, 16)), (18, 18));
    for i in [16, 17] {
        assert!(translated_by_some(&[u], 0, nth(i), &bridges), "scope {}", i + 1);
    }
}

#[test]
fn exactly_255_is_kept_whole() {
    assert_eq!(MAX_SCOPES, 255);
    let u = parse(&endpoints(255));
    assert!(!u.truncated);
    assert_eq!(u.count, 255);
    assert_eq!(scope_covers(&u, 0, nth(254), &bridges), Cover::Yes);
    assert!(translated_by_some(&[u], 0, nth(254), &bridges));
}

#[test]
fn past_255_the_unit_is_truncated_and_the_rest_refused() {
    let b = endpoints(256);
    let u = parse(&b);
    assert!(u.truncated);
    assert_eq!((u.count, pci_scopes_named(&b, 16)), (255, 256), "count never wraps");
    assert_eq!(scope_covers(&u, 0, nth(255), &bridges), Cover::Unknown);
    assert!(!translated_by_some(&[u], 0, nth(255), &bridges));
}

#[test]
fn a_buffer_past_255_still_keeps_no_more_than_a_u8_counts() {
    let b = endpoints(256);
    let mut out = [Scope::EMPTY; 300];
    assert_eq!(walk_scopes(&b, 16, &mut out), (255, true), "never wraps to 0");
}
