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

//! Where a guest may reach out to: anywhere with NET_ANON and no list, only
//! what its list allows with one, and nothing when its list is not held.

use super::app_reach_write::{held, names};
use crate::app::identity::AppIdentity;
use crate::app::reach::{may_dial, reach_of, ANYWHERE};
use nonos_linux_policy::dests::{dests_hash, Target as T};
use nonos_linux_policy::mask::{KNOWN, NET_ANON, NET_LISTEN_LO, TTY};
use nonos_linux_policy::{unlisted, LinuxAppPolicy};

#[test]
fn no_list_reaches_anywhere_with_net_anon_alone() {
    let (mask, lists) = reach_of(NET_ANON, &[0; 32], None);
    assert_eq!((mask, lists), (NET_ANON, ANYWHERE));
    assert!(may_dial(mask, &lists, T::V4([192, 0, 2, 1]), 443));
    assert!(may_dial(KNOWN, &lists, T::Name(b"example.org"), 1));
    assert!(!may_dial(NET_LISTEN_LO | TTY, &lists, T::V4([192, 0, 2, 1]), 443));
}

#[test]
fn a_held_list_is_all_a_guest_reaches() {
    let g = held(1, names(&["api.example.org"]));
    assert_eq!(g.mask, NET_ANON | TTY);
    assert!(may_dial(g.mask, &g.reach, T::Name(b"api.example.org"), 443));
    assert!(!may_dial(g.mask, &g.reach, T::Name(b"api.example.org"), 80));
    assert!(!may_dial(g.mask, &g.reach, T::Name(b"example.org"), 443));
    assert!(!may_dial(g.mask, &g.reach, T::V4([192, 0, 2, 1]), 443));
    assert!(!may_dial(TTY, &g.reach, T::Name(b"api.example.org"), 443));
}

#[test]
fn a_list_not_held_takes_net_anon_away() {
    let list = names(&["api.example.org"]);
    let hash = dests_hash(list);
    assert_eq!(reach_of(NET_ANON | TTY, &hash, None), (TTY, ANYWHERE));
    let other = names(&["other.example.org"]);
    assert_eq!(reach_of(NET_ANON | TTY, &hash, Some(other)), (TTY, ANYWHERE));
    // Bytes that hash right but are not canonical: a byte after the last entry.
    let mut bad = list.to_vec();
    bad.push(0);
    let bad: &'static [u8] = Box::leak(bad.into_boxed_slice());
    assert_eq!(reach_of(NET_ANON, &dests_hash(bad), Some(bad)), (0, ANYWHERE));
    let p = LinuxAppPolicy { mask: NET_ANON, net_dests_hash: hash, ..unlisted([2; 32]) };
    assert_eq!(AppIdentity::of(p).mask, 0);
}
