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

//! The statement's hashes against the gates' own.

use nonos_attest_path::{digest_to_bytes, leaf_of, Kind, Poseidon as GateHasher};

use super::fixture::fixture;
use crate::domain::{KIND_BOOTLOADER, KIND_KERNEL};
use crate::native::{leaf, scope, words_of};

/// The circuit's first compression is the gate's leaf, for both kinds, so the
/// proof opens the very trees the gates check.
#[test]
fn the_first_compression_is_the_gates_leaf() {
    let d = [0xa5u8; 32];
    let h = GateHasher::new();
    for (k, kind) in [(KIND_KERNEL, Kind::Kernel), (KIND_BOOTLOADER, Kind::Bootloader)] {
        let gate = digest_to_bytes(&leaf_of(&h, kind, &d));
        assert_eq!(leaf(k, &d), words_of(&gate));
    }
}

#[test]
fn the_fixture_satisfies_its_statement() {
    let f = fixture();
    assert!(crate::prove::check_for_test(&f.st, &f.w).is_ok());
}

/// Two verifiers, or two windows of one verifier, see unrelated scopes.
#[test]
fn scopes_are_per_verifier_and_per_window() {
    let a = scope(b"faucet", 1).expect("a");
    assert_ne!(a, scope(b"faucet", 2).expect("b"));
    assert_ne!(a, scope(b"relayer", 1).expect("c"));
    assert_ne!(scope(b"ab", 1), scope(b"a", u64::from_le_bytes(*b"b\0\0\0\0\0\0\0")));
}
