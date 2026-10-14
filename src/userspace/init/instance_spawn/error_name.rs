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

use crate::kernel_core::process_spawn::capsule_spawn::SpawnError;

pub(super) fn spawn_error_name(e: SpawnError) -> &'static [u8] {
    match e {
        SpawnError::FeatureDisabled => b"FeatureDisabled",
        SpawnError::ElfLoad => b"ElfLoad",
        SpawnError::ProcessCreation => b"ProcessCreation",
        SpawnError::AddressSpace => b"AddressSpace",
        SpawnError::EndpointCollision => b"EndpointCollision",
        SpawnError::InboxName => b"InboxName",
        SpawnError::NonosIdCertRejected(_) => b"CertRejected",
        SpawnError::ManifestRejected(_) => b"ManifestRejected",
        SpawnError::AttestationRejected => b"AttestationRejected",
        SpawnError::ProfileRefused => b"ProfileRefused",
        SpawnError::ImageCeiling => b"ImageCeiling",
    }
}
