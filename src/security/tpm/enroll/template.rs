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

//! The EK templates of the TCG EK Credential Profile 2.0, low range: L-1, RSA
//! 2048, and L-2, ECC NIST P-256. A TPM derives its EK from the endorsement
//! seed and the template, so `TPM2_CreatePrimary` with exactly these bytes
//! reproduces the key the manufacturer certified, on every call.

use alloc::vec::Vec;

use super::consts::{
    EK_ATTRIBUTES, POLICY_A, TPM_ALG_AES, TPM_ALG_CFB, TPM_ALG_ECC, TPM_ALG_RSA, TPM_ECC_NIST_P256,
};
use super::kind::EkKind;
use crate::security::tpm::machine_key::consts::{DIGEST_LEN, TPM_ALG_NULL, TPM_ALG_SHA256};

fn put(p: &mut Vec<u8>, fields: &[u16]) {
    for v in fields {
        p.extend_from_slice(&v.to_be_bytes());
    }
}

/// TPMT_PUBLIC up to `unique`. The TPM's answer must begin with exactly this.
pub(super) fn ek_shape(kind: EkKind) -> Vec<u8> {
    let alg = if kind == EkKind::Rsa2048 { TPM_ALG_RSA } else { TPM_ALG_ECC };
    let mut p = Vec::with_capacity(64);
    put(&mut p, &[alg, TPM_ALG_SHA256]);
    p.extend_from_slice(&EK_ATTRIBUTES.to_be_bytes());
    put(&mut p, &[DIGEST_LEN as u16]);
    p.extend_from_slice(&POLICY_A);
    /* symmetric AES-128-CFB, scheme null */
    put(&mut p, &[TPM_ALG_AES, 128, TPM_ALG_CFB, TPM_ALG_NULL]);
    match kind {
        /* keyBits 2048, then the 32-bit exponent, 0 meaning 2^16 + 1 */
        EkKind::Rsa2048 => {
            put(&mut p, &[2048]);
            p.extend_from_slice(&0u32.to_be_bytes());
        }
        /* curveID P-256, kdf null */
        EkKind::EccP256 => put(&mut p, &[TPM_ECC_NIST_P256, TPM_ALG_NULL]),
    }
    p
}

/// The whole template: the shape, then `unique` zero-filled at the key's
/// sizes, as L-1 and L-2 specify.
pub(super) fn ek_template(kind: EkKind) -> Vec<u8> {
    let mut p = ek_shape(kind);
    for &n in kind.unique() {
        put(&mut p, &[n as u16]);
        p.resize(p.len() + n, 0);
    }
    p
}
