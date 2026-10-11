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

//! Which policy binds a program: the vendor's table first, then a vouched
//! file, then the owner's default; a bad file refuses the program.

use crate::app_choose::choose;
use nonos_linux_policy::mask::{MODELS_RO, NET_ANON, WAYLAND};
use nonos_linux_policy::{unlisted, LinuxAppPolicy, SyscallSet, UNLISTED_MASK};

fn policy(program: u8, mask: u64) -> LinuxAppPolicy {
    LinuxAppPolicy { mask, syscall_set: SyscallSet::NONE.with(1), ..unlisted([program; 32]) }
}

fn table(ps: &[LinuxAppPolicy]) -> Vec<u8> {
    ps.iter().flat_map(|p| p.encode()).collect()
}

#[test]
fn the_vendor_table_names_its_own_programs() {
    let t = table(&[policy(1, NET_ANON), policy(2, WAYLAND | MODELS_RO)]);
    assert_eq!(choose(&[2; 32], &t, None), Ok(policy(2, WAYLAND | MODELS_RO)));
    let file = policy(2, NET_ANON).encode();
    assert_eq!(choose(&[2; 32], &t, Some(&file)), Ok(policy(2, WAYLAND | MODELS_RO)));
}

#[test]
fn a_vouched_file_binds_a_program_the_table_does_not_name() {
    let t = table(&[policy(1, NET_ANON)]);
    let file = policy(3, WAYLAND).encode();
    assert_eq!(choose(&[3; 32], &t, Some(&file)), Ok(policy(3, WAYLAND)));
}

#[test]
fn no_policy_at_all_is_the_owners_default() {
    let p = choose(&[4; 32], &[], None).unwrap();
    assert_eq!(p, unlisted([4; 32]));
    assert_eq!(p.mask, UNLISTED_MASK);
}

#[test]
fn a_bad_file_refuses_rather_than_defaults() {
    let other = policy(5, WAYLAND).encode();
    assert_eq!(choose(&[6; 32], &[], Some(&other)), Err("its policy names another program"));
    let mut bad = policy(6, WAYLAND).encode();
    bad[230] = 1;
    assert_eq!(choose(&[6; 32], &[], Some(&bad)), Err("its policy is malformed"));
    assert_eq!(choose(&[6; 32], &[], Some(&bad[..255])), Err("its policy is malformed"));
}

#[test]
fn a_malformed_or_partial_table_entry_binds_nothing() {
    let mut t = table(&[policy(7, WAYLAND)]);
    t[0] = b'X';
    t.extend_from_slice(&policy(8, WAYLAND).encode()[..100]);
    assert_eq!(choose(&[7; 32], &t, None), Ok(unlisted([7; 32])));
    assert_eq!(choose(&[8; 32], &t, None), Ok(unlisted([8; 32])));
}
