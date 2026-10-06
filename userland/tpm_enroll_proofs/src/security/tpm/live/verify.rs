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

//! The registrar's check of the AK's signature, made without the TPM.

use p256::ecdsa::signature::Verifier;
use p256::ecdsa::{Signature, VerifyingKey};

use crate::security::tpm::enroll::AK_SIGN_LABEL;

/// What the AK signs for `msg`: the enrollment label, then the message.
pub fn labelled(msg: &[u8]) -> Vec<u8> {
    [AK_SIGN_LABEL, msg].concat()
}

/// ECDSA P-256 over SHA-256 of `msg`, checked with p256 against the point in
/// the AK's TPM2B_PUBLIC, whose `unique` closes it: x and y, 32 bytes each,
/// each after its size.
pub fn verifies(ak_area: &[u8], msg: &[u8], sig: &[u8; 64]) -> bool {
    let n = ak_area.len();
    assert_eq!(ak_area[n - 68..n - 66], [0, 32]);
    assert_eq!(ak_area[n - 34..n - 32], [0, 32]);
    let sec1 = [&[0x04][..], &ak_area[n - 66..n - 34], &ak_area[n - 32..]].concat();
    let key = VerifyingKey::from_sec1_bytes(&sec1).expect("ak point on the curve");
    let sig = Signature::from_slice(sig).expect("r and s in range");
    key.verify(msg, &sig).is_ok()
}
