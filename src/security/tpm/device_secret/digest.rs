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

//! The digest the release key signs and VerifySignature checks. The approved
//! policy under it is the TPM's own: PolicyPCR over PCR 9 from an empty session,
//! read back with PolicyGetDigest, so the kernel computes no policy digest. The
//! release tool computes the same value from the release
//! (`tools/nonos-policy-approve`), and `tpm_enroll_proofs` holds the two equal
//! on swtpm.

use alloc::vec::Vec;

use super::consts::DIGEST_LEN;
use crate::crypto::hash::sha256;

/// `H(approvedPolicy || policyRef)`, TPM 2.0 Part 3, 23.16 (PolicyAuthorize).
pub(super) fn a_hash(approved: &[u8; DIGEST_LEN], policy_ref: &[u8]) -> [u8; DIGEST_LEN] {
    let mut m = Vec::with_capacity(DIGEST_LEN + policy_ref.len());
    m.extend_from_slice(approved);
    m.extend_from_slice(policy_ref);
    sha256(&m)
}
