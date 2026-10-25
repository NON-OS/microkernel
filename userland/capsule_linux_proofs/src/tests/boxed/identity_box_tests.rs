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

//! A box only ever takes away from a program with a published policy: from
//! its own grants, and from every program it execs. The one widening, for a
//! program with none, is identity_widen_tests.rs.

use crate::app::identity::AppIdentity;
use crate::boxed::preset::parse;
use nonos_linux_policy::dests::Target;
use nonos_linux_policy::mask::{EXEC, FILES_HOME, KNOWN, NET_ANON, TTY, WAYLAND};
use nonos_linux_policy::{unlisted, LinuxAppPolicy};

/// A published policy granting `mask`: it names its listing.
pub fn granting(mask: u64) -> AppIdentity {
    AppIdentity::of(LinuxAppPolicy { mask, listing_id: [1; 32], ..unlisted([7; 32]) })
}

pub fn dial(id: &AppIdentity, ip: [u8; 4], port: u16) -> bool {
    crate::app::reach::may_dial(id.mask, &id.reach, Target::V4(ip), port)
}

pub fn leak(v: Vec<u8>) -> &'static [u8] {
    Vec::leak(v)
}

#[test]
fn a_box_never_grants_what_the_program_was_not_granted() {
    for word in [&b"strict"[..], b"offline", b"allow:/tmp/a,net:10.0.0.1:443"] {
        let p = parse(word).unwrap();
        for own in [0, TTY, TTY | FILES_HOME | EXEC, KNOWN] {
            let boxed = granting(own).boxed(p.mask, leak(p.dests.clone()));
            assert_eq!(boxed.mask & !(own & p.mask), 0);
        }
    }
}

#[test]
fn strict_takes_the_network_and_the_display_from_a_program_that_had_them() {
    let p = parse(b"strict").unwrap();
    let boxed = granting(KNOWN).boxed(p.mask, leak(p.dests.clone()));
    assert_eq!(boxed.mask, TTY | EXEC);
    assert_eq!(boxed.mask & (NET_ANON | WAYLAND), 0);
}

#[test]
fn an_allow_box_holds_streams_to_its_destinations_through_an_exec() {
    let p = parse(b"allow:net:10.0.0.1:443").unwrap();
    let boxed = granting(KNOWN).boxed(p.mask, leak(p.dests.clone()));
    assert!(dial(&boxed, [10, 0, 0, 1], 443));
    assert!(!dial(&boxed, [10, 0, 0, 1], 80) && !dial(&boxed, [10, 0, 0, 2], 443));
    let child = boxed.exec(&granting(KNOWN));
    assert!(dial(&child, [10, 0, 0, 1], 443) && !dial(&child, [10, 0, 0, 2], 443));
    assert_eq!(child.mask & !boxed.mask, 0);
}
