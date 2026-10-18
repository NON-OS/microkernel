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

use super::token::{registered_syscalls, token};
use crate::capabilities::Capability;
use crate::syscall::contract::is_allowed;
use crate::syscall::numbers::SyscallNumber;

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

#[test]
fn an_empty_token_is_denied_every_syscall_but_tty_query() {
    // MkTtyQuery asks only about the caller's own streams, so the cap table
    // grants it to anyone. A second call passing here is a hole.
    let empty = token(vec![]);
    let passed: Vec<SyscallNumber> =
        registered_syscalls().into_iter().filter(|&n| is_allowed(&empty, n)).collect();
    assert_eq!(passed, vec![SyscallNumber::MkTtyQuery]);
}

#[test]
fn the_registry_is_what_the_gate_is_checked_over() {
    let calls = registered_syscalls();
    assert!(calls.contains(&SyscallNumber::MkTtyQuery));
    assert!(calls.contains(&SyscallNumber::CryptoRandom));
    assert!(calls.len() > 100, "{} calls", calls.len());
}

#[test]
fn crypto_syscalls_require_the_crypto_capability() {
    let with_crypto = token(vec![Capability::Crypto]);
    let without_crypto = token(vec![Capability::Hardware]);

    assert!(is_allowed(&with_crypto, SyscallNumber::CryptoRandom));
    assert!(is_allowed(&with_crypto, SyscallNumber::CryptoHash));
    assert!(!is_allowed(&without_crypto, SyscallNumber::CryptoRandom));
    assert!(!is_allowed(&without_crypto, SyscallNumber::CryptoHash));
}

#[test]
fn granting_a_capability_never_removes_access() {
    // Monotonicity: a superset of capabilities allows everything the subset does.
    let subset = token(vec![Capability::Crypto]);
    let superset = token(vec![Capability::Crypto, Capability::Admin, Capability::Memory]);
    for n in registered_syscalls() {
        if is_allowed(&subset, n) {
            assert!(is_allowed(&superset, n), "adding a capability removed access");
        }
    }
}
