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

//! A kernel signed by any key but the tree's kernel key is refused before a
//! signature is checked, and so is a rollback index the record does not name.

use super::kernel_tests::kernel_of;
use super::sealed_fixture::Sealed;
use crate::check::Status;
use crate::release::kernel_keys::{ED_PUB, PQ_PUB};
use ed25519_dalek::SigningKey;
use nonos_capsule_sign::algs::{mldsa65::MlDsa65, AlgId, Signer};
use nonos_capsule_sign::keys::write_pub;
use serde_json::json;

#[test]
fn a_kernel_signed_by_another_key_is_refused() {
    let s = Sealed::dev();
    let other = SigningKey::from_bytes(&[9; 32]).verifying_key().to_bytes();
    std::fs::write(s.tree.path().join(ED_PUB), other).unwrap();
    let c = kernel_of(&s);
    assert!(
        c.status == Status::Fail && c.reason.contains("Ed25519 signature is by a key"),
        "{c:?}"
    );
    let s = Sealed::dev();
    let other = MlDsa65::keygen_random().unwrap().pubkey;
    write_pub(&s.tree.path().join(PQ_PUB), AlgId::MlDsa65, &other).unwrap();
    let c = kernel_of(&s);
    assert!(
        c.status == Status::Fail && c.reason.contains("ML-DSA-65 signature is by a key"),
        "{c:?}"
    );
}

#[test]
fn a_key_file_of_the_wrong_kind_is_refused() {
    let s = Sealed::dev();
    let ed = SigningKey::from_bytes(&[9; 32]).verifying_key().to_bytes();
    write_pub(&s.tree.path().join(PQ_PUB), AlgId::Ed25519, &ed).unwrap();
    let c = kernel_of(&s);
    assert!(c.status == Status::Fail && c.reason.contains("is not a mldsa65 key"), "{c:?}");
    // The Ed25519 key in any form but the raw one the seal writes.
    let s = Sealed::dev();
    write_pub(&s.tree.path().join(ED_PUB), AlgId::Ed25519, &ed).unwrap();
    let c = kernel_of(&s);
    assert!(c.status == Status::Fail && c.reason.contains("not the raw 32-byte key"), "{c:?}");
}

#[test]
fn a_rollback_index_the_record_does_not_name_is_refused() {
    let mut s = Sealed::dev();
    s.release.config["rollback_index"] = json!(2);
    s.release.record();
    let c = kernel_of(&s);
    assert!(c.status == Status::Fail && c.reason.contains("rollback index"), "{c:?}");
}
