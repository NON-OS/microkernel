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
fn an_empty_token_is_denied_every_syscall_but_tty_query_and_random() {
    // MkTtyQuery asks only about the caller's own streams, and random bytes
    // are every process's (a project decision, 2026-11-02), so the cap table
    // grants both to anyone. A third call passing here is a hole.
    let empty = token(vec![]);
    let mut passed: Vec<SyscallNumber> =
        registered_syscalls().into_iter().filter(|&n| is_allowed(&empty, n)).collect();
    passed.sort_by_key(|n| *n as u64);
    let mut want = vec![SyscallNumber::MkTtyQuery, SyscallNumber::CryptoRandom];
    want.sort_by_key(|n| *n as u64);
    assert_eq!(passed, want);
}

#[test]
fn the_registry_is_what_the_gate_is_checked_over() {
    let calls = registered_syscalls();
    assert!(calls.contains(&SyscallNumber::MkTtyQuery));
    assert!(calls.contains(&SyscallNumber::CryptoRandom));
    assert!(calls.len() > 100, "{} calls", calls.len());
}

#[test]
fn crypto_syscalls_but_random_require_the_crypto_capability() {
    let with_crypto = token(vec![Capability::Crypto]);
    let without_crypto = token(vec![Capability::Hardware]);

    assert!(is_allowed(&with_crypto, SyscallNumber::CryptoRandom));
    assert!(is_allowed(&with_crypto, SyscallNumber::CryptoHash));
    assert!(is_allowed(&without_crypto, SyscallNumber::CryptoRandom));
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

#[test]
fn a_quote_is_withdrawn_from_a_token_that_reaches_the_network() {
    // The one place a wider token holds less: a TPM quote names the machine
    // for good, so MkAttestDoc goes to AttestRead only without Network. The
    // syscall_gate fuzz target holds every other call to monotonicity.
    let quiet = token(vec![Capability::AttestRead]);
    let networked = token(vec![Capability::AttestRead, Capability::Network]);
    assert!(is_allowed(&quiet, SyscallNumber::MkAttestDoc));
    assert!(!is_allowed(&networked, SyscallNumber::MkAttestDoc));
    for n in registered_syscalls() {
        if n != SyscallNumber::MkAttestDoc && is_allowed(&quiet, n) {
            assert!(is_allowed(&networked, n), "{n:?} lost by adding Network");
        }
    }
}
