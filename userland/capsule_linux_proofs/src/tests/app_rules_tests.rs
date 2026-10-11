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

//! The rules each gate asks before a guest is served: the declared calls,
//! an internet socket, and a stream out of the family.

use crate::app::rules::{call_refused, inet_allowed, may_dial};
use nonos_linux_policy::mask::{KNOWN, NET_ANON, NET_LISTEN_LO, TTY, WAYLAND};
use nonos_linux_policy::SyscallSet;

#[test]
fn a_call_outside_the_set_is_refused_and_one_inside_is_not() {
    let set = SyscallSet::NONE.with(0).with(1).with(9999);
    assert!(!call_refused(&set, 0) && !call_refused(&set, 1));
    assert!(call_refused(&set, 2) && call_refused(&set, 511));
    // every number past 511 shares the last bucket
    assert!(!call_refused(&set, 512) && !call_refused(&set, u64::MAX));
    assert!(call_refused(&SyscallSet::NONE, 512));
}

#[test]
fn exit_and_exit_group_are_never_refused() {
    // x86_64 exit and exit_group
    for nr in [60, 231] {
        assert!(!call_refused(&SyscallSet::NONE, nr), "{nr}");
    }
    assert!(call_refused(&SyscallSet::NONE, 59) && call_refused(&SyscallSet::NONE, 61));
}

#[test]
fn every_call_is_allowed_under_the_full_set() {
    assert!((0..1024).all(|nr| !call_refused(&SyscallSet::ALL, nr)));
}

#[test]
fn an_internet_socket_needs_one_of_the_network_bits() {
    assert!(inet_allowed(NET_ANON) && inet_allowed(NET_LISTEN_LO));
    assert!(!inet_allowed(0) && !inet_allowed(TTY | WAYLAND));
    assert!(inet_allowed(KNOWN));
}

#[test]
fn reaching_out_needs_net_anon_and_no_destination_list() {
    assert!(may_dial(NET_ANON, &[0; 32]));
    assert!(!may_dial(NET_LISTEN_LO, &[0; 32]) && !may_dial(0, &[0; 32]));
    // Any byte of the 32 names a list, which is refused until C7 reads it.
    for at in [0, 7, 8, 31] {
        let mut named = [0; 32];
        named[at] = 1;
        assert!(!may_dial(NET_ANON, &named) && !may_dial(KNOWN, &named), "byte {at}");
    }
}
