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

use super::error::AttestError;

/// Verify a capsule against the vendor's root. Only a v4 trailer opens it, path
/// and STARK both: letting the trailer choose its verifier would let a prover
/// pick a weaker one for the root everything shipped is measured under.
pub(super) fn vendor(
    trailer: &[u8],
    digest: &[u8; 32],
    granted_caps: u64,
    root: &[u8; 32],
) -> Result<[u8; 32], AttestError> {
    super::path::verify_against(trailer, digest, granted_caps, root)
}

/// Verify against a root a person enrolled on this machine: a developer's
/// path tree, or this machine's own local root, whose tags only this kernel
/// can check.
pub(super) fn enrolled(
    trailer: &[u8],
    digest: &[u8; 32],
    granted_caps: u64,
    root: &[u8; 32],
) -> Result<[u8; 32], AttestError> {
    if trailer.starts_with(crate::security::local_build::MAGIC) {
        return crate::security::local_build::verify(trailer, digest, granted_caps, root)
            .ok_or(AttestError::Rejected);
    }
    super::path::verify_against(trailer, digest, granted_caps, root)
}
