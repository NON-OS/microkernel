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

use crate::memory::iommu::{DeviceAddress, IommuCapabilities, IommuVendor};

use super::super::domain::MAX_DOMAINS;
use super::super::encode::cd::T0SZ;
use super::super::state::SMMU;

pub fn select_vendor() -> IommuVendor {
    match SMMU.lock().is_some() {
        true => IommuVendor::ArmSmmuV3,
        false => IommuVendor::Absent,
    }
}

/// Only what is in force: nothing until bring-up succeeded, then the
/// stage 1 tables' reach and the 4 KiB leaves the driver installs.
pub fn capabilities() -> IommuCapabilities {
    if SMMU.lock().is_none() {
        return IommuCapabilities::none_in_force(IommuVendor::Absent);
    }
    IommuCapabilities {
        vendor: IommuVendor::ArmSmmuV3,
        enforcing: true,
        address_width_bits: 64 - T0SZ as u8,
        interrupt_remapping: false,
        page_sizes: 1 << 12,
        snoop_control: true,
        domain_count: MAX_DOMAINS as u32,
    }
}

pub fn translates(device: DeviceAddress) -> bool {
    SMMU.lock().as_ref().is_some_and(|smmu| smmu.stream(device).is_some())
}

/// Every stream no capsule's domain has claimed aborts: the driver never
/// writes a bypass STE, so no device reaches memory around a domain.
pub fn identity_devices() -> usize {
    0
}
