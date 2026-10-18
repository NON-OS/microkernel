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

//! The identity a guest carries, and that an exec never widens its box.

use crate::app::identity::AppIdentity;
use nonos_linux_policy::mask::{EXEC, KNOWN, NET_ANON, TTY, WAYLAND};
use nonos_linux_policy::{
    app_id, policy_hash, unlisted, LinuxAppPolicy, SyscallSet, UNLISTED_MASK,
};

fn policy(program: u8, mask: u64, calls: SyscallSet) -> LinuxAppPolicy {
    LinuxAppPolicy { mask, syscall_set: calls, ..unlisted([program; 32]) }
}

#[test]
fn of_names_the_program_and_its_policy() {
    let p = policy(1, WAYLAND, SyscallSet::NONE.with(0));
    let a = AppIdentity::of(p);
    assert_eq!(a.policy_hash, policy_hash(&p.encode()));
    assert_eq!(a.app_id, app_id(&[1; 32], &a.policy_hash));
    assert_eq!((a.mask, a.calls), (WAYLAND, p.syscall_set));
}

#[test]
fn a_guest_with_no_program_is_served_nothing() {
    let n = AppIdentity::NOTHING;
    assert_eq!(n.mask, 0);
    assert!((0..600).all(|nr| !n.calls.allows(nr)));
}

#[test]
fn an_exec_takes_the_new_identity_inside_both_boxes() {
    let shell = AppIdentity::of(policy(1, TTY | EXEC, SyscallSet::ALL));
    let net = AppIdentity::of(policy(2, KNOWN, SyscallSet::NONE.with(41).with(42)));
    let after = shell.exec(&net);
    assert_eq!((after.app_id, after.policy), (net.app_id, net.policy));
    assert_eq!(after.mask, TTY | EXEC);
    assert_eq!(after.calls, SyscallSet::NONE.with(41).with(42));
}

#[test]
fn a_chain_of_execs_never_widens() {
    let mut g = AppIdentity::of(policy(1, TTY, SyscallSet::NONE.with(1)));
    for wide in [KNOWN, NET_ANON | TTY, KNOWN] {
        g = g.exec(&AppIdentity::of(policy(3, wide, SyscallSet::ALL)));
        assert_eq!(g.mask & !TTY, 0);
        assert!(g.calls.allows(1) && !g.calls.allows(2));
    }
}

#[test]
fn the_built_in_runs_in_the_default_box_with_no_app_id() {
    let b = AppIdentity::built_in([7; 32]);
    assert_eq!(b.policy, unlisted([7; 32]));
    assert_eq!((b.app_id, b.mask, b.calls), ([0; 32], UNLISTED_MASK, SyscallSet::ALL));
    let vendor = AppIdentity::of(policy(2, KNOWN, SyscallSet::ALL));
    assert_eq!(vendor.exec(&b).app_id, [0; 32]);
    assert_eq!(b.exec(&vendor).mask, UNLISTED_MASK);
}
