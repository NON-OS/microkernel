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

use super::super::error::ManifestVerifyError;
use super::super::schema::CapsuleManifest;

/*
 * The payload is the capsule's whole image, megabytes. Its BLAKE3 is the
 * measurement the attestation gate also checks, so the spawn path takes it
 * once (`capsule_attest::measure`) and both checks compare against it.
 */
pub(super) fn check(manifest: &CapsuleManifest, digest: &[u8; 32]) -> Result<(), ManifestVerifyError> {
    if *digest != manifest.payload_hash {
        return Err(ManifestVerifyError::PayloadHashMismatch);
    }
    Ok(())
}
