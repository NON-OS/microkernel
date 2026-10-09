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

//! Whether a claim may go ahead with its device unconfined. Pure, and proven on
//! the host (kernel_proofs::confine_posture).
//!
//! The one case is that no remapping unit is in service: none was found, the
//! one found never came up, or it is AMD-Vi, which this kernel does not drive.
//! The device then reaches all of memory whatever the broker does, and the
//! claim goes ahead with the boot log saying so. Every other failure to give a
//! device its own domain happens with a unit in service, and there the claim
//! is refused: granting it would hand a capsule a device the unit was ready to
//! confine and did not.

use crate::memory::iommu::IommuError;

/// True only for the errors that mean no unit is in service. Exhaustive, with
/// no catch-all, so a new error has to be placed on one side or the other.
pub(super) const fn unconfined_allowed(e: IommuError) -> bool {
    match e {
        IommuError::NotInitialized
        | IommuError::NotSupported
        | IommuError::NoIommu
        | IommuError::AmdViNotDriven => true,
        IommuError::DomainExhausted
        | IommuError::InvalidDomain
        | IommuError::InvalidDevice
        | IommuError::InvalidIova
        | IommuError::InvalidSize
        | IommuError::AlreadyMapped
        | IommuError::NotMapped
        | IommuError::DeviceAttachFailed
        | IommuError::DeviceDetachFailed
        | IommuError::PageTableExhausted
        | IommuError::BackendFault => false,
    }
}
