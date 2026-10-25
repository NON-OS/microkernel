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

//! A key the tree's trust set publishes is refused though it lies anywhere,
//! a key that is not an Ed25519 seed is refused, and a `.keys` of the
//! operator's own, outside any NONOS checkout, is theirs to use.

use super::fake::Fake;
use super::sealed_fixture::Sealed;
use super::sign_refuse_tests::refused;
use super::signing::{operator, signed};
use crate::exit;
use crate::tree::ledger::TRUST;
use nonos_capsule_sign::{algs::AlgId, keys::write_pub};

#[test]
fn a_key_the_trust_set_publishes_is_refused() {
    let (s, dir) = (Sealed::dev(), tempfile::tempdir().unwrap());
    let (path, key) = operator(dir.path(), 42);
    let published = s.tree.path().join(TRUST).join("keys/about_publisher_ed25519.pub");
    write_pub(&published, AlgId::Ed25519, key.verifying_key().as_bytes()).unwrap();
    refused(&s, &path);
}

#[test]
fn a_key_that_is_not_an_ed25519_seed_is_refused() {
    let (s, dir) = (Sealed::dev(), tempfile::tempdir().unwrap());
    let junk = dir.path().join("junk.seed");
    std::fs::write(&junk, b"not a key at all").unwrap();
    refused(&s, &junk);
}

#[test]
fn a_dot_keys_outside_any_nonos_checkout_is_the_operators_and_is_used() {
    let (s, home) = (Sealed::dev(), tempfile::tempdir().unwrap());
    let (path, _) = operator(&home.path().join(".keys"), 42);
    assert_eq!(signed(&s, &path, true, Fake::default()).0, exit::DEV_GO);
}
