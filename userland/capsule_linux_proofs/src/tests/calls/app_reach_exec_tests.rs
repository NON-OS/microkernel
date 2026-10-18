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

//! An exec never reaches past a list on the way: the new program is held to
//! its own list and every one before it, two at most, or loses NET_ANON.

use super::app_reach_write::{held, names};
use crate::app::identity::AppIdentity;
use crate::app::reach::{chain, may_dial, ANYWHERE};
use nonos_linux_policy::dests::Target as T;
use nonos_linux_policy::mask::{NET_ANON, TTY};
use nonos_linux_policy::{unlisted, LinuxAppPolicy};

fn free(program: u8) -> AppIdentity {
    AppIdentity::of(LinuxAppPolicy { mask: NET_ANON | TTY, ..unlisted([program; 32]) })
}

#[test]
fn a_list_survives_an_exec_into_a_program_without_one() {
    let after = held(1, names(&["a.example"])).exec(&free(2));
    assert_eq!(after.mask, NET_ANON | TTY);
    assert!(may_dial(after.mask, &after.reach, T::Name(b"a.example"), 443));
    assert!(!may_dial(after.mask, &after.reach, T::Name(b"b.example"), 443));
}

#[test]
fn two_lists_allow_only_what_both_allow() {
    let a = held(1, names(&["a.example", "both.example"]));
    let b = held(2, names(&["b.example", "both.example"]));
    let after = a.exec(&b);
    assert!(may_dial(after.mask, &after.reach, T::Name(b"both.example"), 443));
    assert!(!may_dial(after.mask, &after.reach, T::Name(b"a.example"), 443));
    assert!(!may_dial(after.mask, &after.reach, T::Name(b"b.example"), 443));
    // The same list again is held once.
    assert_eq!(after.exec(&b).reach, after.reach);
}

#[test]
fn a_third_list_takes_net_anon_away() {
    let lists = [names(&["a.example"]), names(&["b.example"]), names(&["c.example"])];
    let ab = held(1, lists[0]).exec(&held(2, lists[1]));
    let abc = ab.exec(&held(3, lists[2]));
    assert_eq!((abc.mask, abc.reach), (TTY, ANYWHERE));
    assert_eq!(chain(&ab.reach, &[lists[2], &[]]), None);
    assert_eq!(chain(&ANYWHERE, &ANYWHERE), Some(ANYWHERE));
}
