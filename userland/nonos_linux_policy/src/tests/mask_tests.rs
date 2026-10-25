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

use crate::mask::*;
use crate::{decode, unlisted, SyscallSet, UNLISTED_MASK};

#[test]
fn the_named_bits_are_the_plans() {
    let named =
        [WAYLAND, NET_ANON, NET_LISTEN_LO, FILES_HOME, FILES_USER, MODELS_RO, EXEC, JIT, TTY];
    for (bit, m) in named.iter().enumerate() {
        assert_eq!(*m, 1 << bit);
    }
    assert_eq!(KNOWN, 0x1FF);
}

#[test]
fn an_exec_never_widens_the_box() {
    for parent in [0, TTY, WAYLAND | MODELS_RO, KNOWN] {
        for program in [0, TTY | NET_ANON, KNOWN] {
            let after = narrow(parent, program);
            assert!(grants(parent, after) && grants(program, after));
        }
    }
    assert_eq!(narrow(TTY, KNOWN), TTY);
}

#[test]
fn grants_needs_every_bit() {
    assert!(grants(WAYLAND | TTY, WAYLAND));
    assert!(!grants(WAYLAND, WAYLAND | TTY));
    assert!(grants(0, 0));
}

#[test]
fn a_program_with_no_policy_gets_the_owners_default() {
    assert_eq!(UNLISTED_MASK, TTY | FILES_HOME | EXEC);
    let p = unlisted([7; 32]);
    assert!(!grants(p.mask, WAYLAND) && !grants(p.mask, NET_ANON) && !grants(p.mask, JIT));
    assert_eq!(p.syscall_set, SyscallSet::ALL);
    assert_eq!(decode(&p.encode()), Ok(p));
}
