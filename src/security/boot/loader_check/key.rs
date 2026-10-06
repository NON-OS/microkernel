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

//! The signature on a boot-root record: ECDSA P-256 under the device policy
//! key this kernel was built with, the key `tools/nonos-policy-approve` signs
//! with. All zero when the key bootstrap provisioned none, and then nothing
//! verifies.

use crate::crypto::asymmetric::p256::{verify, PublicKey, Signature};

const POLICY_KEY: &[u8; 64] = include_bytes!(concat!(env!("OUT_DIR"), "/device_policy_p256.pub"));

/// Whether `r`, `s` sign `digest` under the device policy key.
pub(super) fn signed(digest: &[u8; 32], r: &[u8; 32], s: &[u8; 32]) -> bool {
    if POLICY_KEY.iter().all(|&b| b == 0) {
        return false;
    }
    let mut pk: PublicKey = [0u8; 65];
    pk[0] = 0x04;
    pk[1..].copy_from_slice(POLICY_KEY);
    let mut sig: Signature = [0u8; 64];
    sig[..32].copy_from_slice(r);
    sig[32..].copy_from_slice(s);
    verify(&pk, digest, &sig)
}
