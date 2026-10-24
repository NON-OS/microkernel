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

//! Step 4, each half of the hybrid signature alone, and a signed field
//! changed after signing.

use nsh_attest::Refusal;

use super::refused;
use crate::build::Spec;

const ML_SIG: usize = 3309;

#[test]
fn a_bad_ed25519_half_alone_is_refused() {
    let s = Spec {
        cert_bytes: |b| {
            let n = b.len();
            b[n - ML_SIG - 1] ^= 1
        },
        ..Spec::default()
    };
    assert_eq!(refused(s), Refusal::EnrolSignature);
}

#[test]
fn a_bad_ml_dsa_half_alone_is_refused() {
    let s = Spec {
        cert_bytes: |b| {
            let n = b.len();
            b[n - 2000] ^= 1
        },
        ..Spec::default()
    };
    assert_eq!(refused(s), Refusal::EnrolSignature);
}

#[test]
fn a_signed_field_changed_after_signing_is_refused() {
    // not_after_ms, moved later by one: the authority did not sign it.
    let s = Spec {
        cert_bytes: |b| {
            let n = b.len();
            b[n - 3373 - 32 - 8] ^= 1
        },
        ..Spec::default()
    };
    assert_eq!(refused(s), Refusal::EnrolSignature);
}
