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

use crate::security::capsule_manifest::ManifestVerifyError;
use crate::security::nonos_id_cert::IdCertVerifyError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpawnError {
    FeatureDisabled,
    ElfLoad,
    ProcessCreation,
    AddressSpace,
    EndpointCollision,
    /// The reply inbox name is empty or longer than a process stores.
    InboxName,
    NonosIdCertRejected(IdCertVerifyError),
    ManifestRejected(ManifestVerifyError),
    AttestationRejected,
    /* The boot profile chosen in the menu does not run this capsule. */
    ProfileRefused,
    /// The installed caps go past the image ceiling.
    ImageCeiling,
}

impl From<IdCertVerifyError> for SpawnError {
    fn from(e: IdCertVerifyError) -> Self {
        SpawnError::NonosIdCertRejected(e)
    }
}

impl From<ManifestVerifyError> for SpawnError {
    fn from(e: ManifestVerifyError) -> Self {
        SpawnError::ManifestRejected(e)
    }
}
