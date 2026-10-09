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

//! VerifySignature: the release's signature checked by the TPM, for a ticket.

use alloc::vec::Vec;

use super::approval::{Approval, POLICY_REF};
use super::command::{build_load_external, parse_load_external, tpm2b};
use super::digest::a_hash;
use super::public::{p256_name, p256_public};
use super::consts::{COORD_LEN, DIGEST_LEN, TPM_ALG_ECDSA, TPM_ALG_SHA256, TPM_CC_VERIFY_SIGNATURE, TPM_ST_VERIFIED};
use crate::security::tpm::error::TpmError;
use crate::security::tpm::machine_key::consts::TPM_ST_NO_SESSIONS;
use crate::security::tpm::machine_key::flush::build_flush;
use crate::security::tpm::machine_key::run::run;
use crate::security::tpm::machine_key::wire::{checked, frame};
use crate::security::tpm::machine_key::KeyError;

pub(super) fn build_verify_signature(
    key: u32,
    digest: &[u8; DIGEST_LEN],
    r: &[u8; COORD_LEN],
    s: &[u8; COORD_LEN],
) -> Vec<u8> {
    let mut body = Vec::with_capacity(4 + 2 + DIGEST_LEN + 4 + 4 + 2 * COORD_LEN);
    body.extend_from_slice(&key.to_be_bytes());
    tpm2b(&mut body, digest);
    // TPMT_SIGNATURE: ECDSA over SHA-256, then r and s.
    body.extend_from_slice(&TPM_ALG_ECDSA.to_be_bytes());
    body.extend_from_slice(&TPM_ALG_SHA256.to_be_bytes());
    tpm2b(&mut body, r);
    tpm2b(&mut body, s);
    frame(TPM_ST_NO_SESSIONS, TPM_CC_VERIFY_SIGNATURE, &body)
}

/// The TPMT_TK_VERIFIED ticket, whole, as PolicyAuthorize takes it back.
pub(super) fn parse_verify_signature(resp: &[u8]) -> Result<Vec<u8>, KeyError> {
    let r = checked(resp)?;
    let tag = r.get(10..12).ok_or(TpmError::InvalidResponse)?;
    if u16::from_be_bytes([tag[0], tag[1]]) != TPM_ST_VERIFIED {
        return Err(TpmError::InvalidResponse.into());
    }
    let size = r.get(16..18).ok_or(TpmError::InvalidResponse)?;
    let end = 18 + u16::from_be_bytes([size[0], size[1]]) as usize;
    Ok(r.get(10..end).ok_or(TpmError::InvalidResponse)?.to_vec())
}

/*
 * The release's signature over this kernel's approved policy, checked by the
 * TPM, whose name for the loaded key must be the one the policy binds.
 */
pub(super) fn verified(a: &Approval, approved: &[u8; 32]) -> Result<Vec<u8>, KeyError> {
    let (handle, name) = parse_load_external(&run(&build_load_external(&p256_public(&a.key_x, &a.key_y)))?)?;
    let digest = a_hash(approved, POLICY_REF);
    let ticket = run(&build_verify_signature(handle, &digest, &a.sig_r, &a.sig_s)).and_then(|r| parse_verify_signature(&r));
    let _ = run(&build_flush(handle));
    if name[..] != p256_name(&a.key_x, &a.key_y)[..] {
        return Err(TpmError::InvalidResponse.into());
    }
    ticket
}
