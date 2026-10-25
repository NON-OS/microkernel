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

//! The one widening a box makes: a program with no policy of its own, in a
//! box naming destinations, may reach those destinations and no other; so
//! may each such program it execs. A published policy, a box with no
//! destination, and a program the box did not start are never widened.

use super::identity_box_tests::{dial, granting, leak};
use crate::app::identity::AppIdentity;
use crate::boxed::preset::parse;
use nonos_linux_policy::mask::NET_ANON;
use nonos_linux_policy::{unlisted, LinuxAppPolicy, UNLISTED_MASK};

fn own_default() -> AppIdentity {
    AppIdentity::of(unlisted([9; 32]))
}
fn boxed(app: AppIdentity, word: &[u8]) -> AppIdentity {
    let p = parse(word).unwrap();
    app.boxed(p.mask, leak(p.dests.clone()))
}

#[test]
fn a_program_with_no_policy_reaches_the_box_destinations_only() {
    let b = boxed(own_default(), b"allow:/tmp/o,net:10.0.0.1:443");
    assert_eq!(b.mask & NET_ANON, NET_ANON);
    assert!(dial(&b, [10, 0, 0, 1], 443));
    assert!(!dial(&b, [10, 0, 0, 1], 80) && !dial(&b, [10, 0, 0, 2], 443));
}

#[test]
fn a_published_policy_is_never_widened() {
    for own in [0, UNLISTED_MASK] {
        let b = boxed(granting(own), b"allow:/tmp/o,net:10.0.0.1:443");
        assert_eq!(b.mask & NET_ANON, 0, "a vendor policy without NET_ANON keeps none");
        assert!(!dial(&b, [10, 0, 0, 1], 443));
    }
    let named = AppIdentity::of(LinuxAppPolicy { release_id: [2; 32], ..unlisted([9; 32]) });
    assert_eq!(named.widened(b"\0\x01").mask, named.mask);
}

#[test]
fn a_box_without_destinations_never_widens() {
    for word in [&b"strict"[..], b"offline", b"allow:/tmp/o"] {
        assert_eq!(boxed(own_default(), word).mask & NET_ANON, 0);
    }
    assert_eq!(own_default().widened(&[]), own_default());
}

#[test]
fn an_exec_widened_by_the_box_stays_on_its_list() {
    let p = parse(b"allow:/tmp/o,net:10.0.0.1:443").unwrap();
    let b = own_default().boxed(p.mask, leak(p.dests.clone()));
    let child = b.exec(&own_default().widened(&p.dests));
    assert!(dial(&child, [10, 0, 0, 1], 443) && !dial(&child, [10, 0, 0, 2], 443));
    assert_eq!(child.mask & !b.mask, 0);
    let outside = boxed(own_default(), b"strict").exec(&own_default().widened(&p.dests));
    assert_eq!(outside.mask & NET_ANON, 0, "an exec never gives what its parent lacks");
}
