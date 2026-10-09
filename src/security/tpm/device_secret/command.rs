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

//! LoadExternal and PolicyAuthorize, built and parsed. Every read of a
//! response is checked; nothing here panics on what a part returns.

use alloc::vec::Vec;

use super::consts::{DIGEST_LEN, TPM_CC_LOAD_EXTERNAL, TPM_CC_POLICY_AUTHORIZE, TPM_RH_OWNER};
use crate::security::tpm::error::TpmError;
use crate::security::tpm::machine_key::consts::TPM_ST_NO_SESSIONS;
use crate::security::tpm::machine_key::wire::{checked, frame, u32_at};
use crate::security::tpm::machine_key::KeyError;

pub(super) fn tpm2b(out: &mut Vec<u8>, bytes: &[u8]) {
    out.extend_from_slice(&(bytes.len() as u16).to_be_bytes());
    out.extend_from_slice(bytes);
}

/*
 * The public half only, in the owner hierarchy. A key loaded under the null
 * hierarchy yields a null ticket from VerifySignature, and PolicyAuthorize
 * refuses a null ticket.
 */
pub(super) fn build_load_external(public: &[u8]) -> Vec<u8> {
    let mut body = Vec::with_capacity(8 + public.len());
    // inPrivate: TPM2B_SENSITIVE, empty.
    body.extend_from_slice(&0u16.to_be_bytes());
    tpm2b(&mut body, public);
    body.extend_from_slice(&TPM_RH_OWNER.to_be_bytes());
    frame(TPM_ST_NO_SESSIONS, TPM_CC_LOAD_EXTERNAL, &body)
}

/// The loaded key's handle and the name the TPM computed for it.
pub(super) fn parse_load_external(resp: &[u8]) -> Result<(u32, Vec<u8>), KeyError> {
    let r = checked(resp)?;
    let handle = u32_at(r, 10)?;
    let size = r.get(14..16).ok_or(TpmError::InvalidResponse)?;
    let n = u16::from_be_bytes([size[0], size[1]]) as usize;
    let name = r.get(16..16 + n).ok_or(TpmError::InvalidResponse)?;
    Ok((handle, name.to_vec()))
}

pub(super) fn build_policy_authorize(
    session: u32,
    approved: &[u8; DIGEST_LEN],
    policy_ref: &[u8],
    key_name: &[u8],
    ticket: &[u8],
) -> Vec<u8> {
    let mut body = Vec::with_capacity(4 + 6 + DIGEST_LEN + policy_ref.len() + key_name.len() + ticket.len());
    body.extend_from_slice(&session.to_be_bytes());
    tpm2b(&mut body, approved);
    tpm2b(&mut body, policy_ref);
    tpm2b(&mut body, key_name);
    body.extend_from_slice(ticket);
    frame(TPM_ST_NO_SESSIONS, TPM_CC_POLICY_AUTHORIZE, &body)
}

pub(super) fn parse_policy_authorize(resp: &[u8]) -> Result<(), KeyError> {
    checked(resp).map(|_| ())
}
