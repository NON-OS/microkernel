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

//! `TPM2_PolicySecret(TPM_RH_ENDORSEMENT)`, which brings a fresh policy
//! session to policy A, the EK's authorization for use. TPM 2.0 Part 3, 23.4.

use alloc::vec::Vec;

use super::consts::{TPM_CC_POLICY_SECRET, TPM_RH_ENDORSEMENT};
use super::error::EnrollError;
use super::marshal::put_password;
use crate::security::tpm::machine_key::consts::TPM_ST_SESSIONS;
use crate::security::tpm::machine_key::wire::{checked, frame};

/// The endorsement hierarchy's empty password authorizes the secret.
pub(in crate::security::tpm) fn build_policy_secret(session: u32) -> Vec<u8> {
    let mut body = Vec::with_capacity(31);
    body.extend_from_slice(&TPM_RH_ENDORSEMENT.to_be_bytes());
    body.extend_from_slice(&session.to_be_bytes());
    put_password(&mut body);
    /* nonceTPM, cpHashA and policyRef empty, expiration zero */
    body.extend_from_slice(&[0; 10]);
    frame(TPM_ST_SESSIONS, TPM_CC_POLICY_SECRET, &body)
}

/// The timeout and ticket after the code are empty for a zero expiration.
pub(in crate::security::tpm) fn parse_policy_secret(resp: &[u8]) -> Result<(), EnrollError> {
    checked(resp)?;
    Ok(())
}
