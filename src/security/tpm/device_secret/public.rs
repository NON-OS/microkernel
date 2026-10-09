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

//! The release policy key as the TPM sees it: a TPMT_PUBLIC for a P-256 ECDSA
//! verification key, and the name `PolicyAuthorize` binds, which is the name
//! algorithm followed by the hash of that structure. The release tool computes
//! the same bytes, so the two agree on the name without a TPM.

use alloc::vec::Vec;

use super::consts::{
    COORD_LEN, P256_ATTRIBUTES, TPM_ALG_ECC, TPM_ALG_ECDSA, TPM_ALG_NULL, TPM_ALG_SHA256, TPM_ECC_NIST_P256,
};
use crate::crypto::hash::sha256;

pub fn p256_public(x: &[u8; COORD_LEN], y: &[u8; COORD_LEN]) -> Vec<u8> {
    let mut p = Vec::with_capacity(96);
    p.extend_from_slice(&TPM_ALG_ECC.to_be_bytes());
    p.extend_from_slice(&TPM_ALG_SHA256.to_be_bytes());
    p.extend_from_slice(&P256_ATTRIBUTES.to_be_bytes());
    // authPolicy: empty.
    p.extend_from_slice(&0u16.to_be_bytes());
    // TPMS_ECC_PARMS: no symmetric, ECDSA over SHA-256, P-256, no KDF.
    p.extend_from_slice(&TPM_ALG_NULL.to_be_bytes());
    p.extend_from_slice(&TPM_ALG_ECDSA.to_be_bytes());
    p.extend_from_slice(&TPM_ALG_SHA256.to_be_bytes());
    p.extend_from_slice(&TPM_ECC_NIST_P256.to_be_bytes());
    p.extend_from_slice(&TPM_ALG_NULL.to_be_bytes());
    // unique: the point.
    p.extend_from_slice(&(COORD_LEN as u16).to_be_bytes());
    p.extend_from_slice(x);
    p.extend_from_slice(&(COORD_LEN as u16).to_be_bytes());
    p.extend_from_slice(y);
    p
}

/// The key's name: `TPM_ALG_SHA256 || SHA-256(TPMT_PUBLIC)`.
pub fn p256_name(x: &[u8; COORD_LEN], y: &[u8; COORD_LEN]) -> [u8; 34] {
    let mut name = [0u8; 34];
    name[..2].copy_from_slice(&TPM_ALG_SHA256.to_be_bytes());
    name[2..].copy_from_slice(&sha256(&p256_public(x, y)));
    name
}
