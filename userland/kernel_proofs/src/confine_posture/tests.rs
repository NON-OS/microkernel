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

use super::posture::unconfined_allowed;
use crate::memory::iommu::IommuError;

const ALL: [IommuError; 15] = [
    IommuError::NotInitialized,
    IommuError::NotSupported,
    IommuError::DomainExhausted,
    IommuError::InvalidDomain,
    IommuError::InvalidDevice,
    IommuError::InvalidIova,
    IommuError::InvalidSize,
    IommuError::AlreadyMapped,
    IommuError::NotMapped,
    IommuError::DeviceAttachFailed,
    IommuError::DeviceDetachFailed,
    IommuError::PageTableExhausted,
    IommuError::BackendFault,
    IommuError::AmdViNotDriven,
    IommuError::NoIommu,
];

#[test]
fn a_device_goes_unconfined_only_when_no_unit_is_in_service() {
    let allowed: std::vec::Vec<IommuError> =
        ALL.iter().copied().filter(|e| unconfined_allowed(*e)).collect();
    assert_eq!(
        allowed,
        [
            IommuError::NotInitialized,
            IommuError::NotSupported,
            IommuError::AmdViNotDriven,
            IommuError::NoIommu
        ]
    );
}

#[test]
fn every_failure_with_a_unit_in_service_refuses_the_claim() {
    for e in [
        IommuError::DomainExhausted,
        IommuError::DeviceAttachFailed,
        IommuError::PageTableExhausted,
        IommuError::BackendFault,
        IommuError::AlreadyMapped,
        IommuError::InvalidDevice,
    ] {
        assert!(!unconfined_allowed(e), "{e:?} would grant an unconfined device");
    }
}
