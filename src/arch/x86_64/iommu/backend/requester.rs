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

//! Which requesters the x86 units can name at all.

use crate::memory::iommu::{DeviceAddress, IommuError};

/// The bus, device and function of a requester in segment 0, the only
/// segment bring-up programs a unit for. Any other segment, and a StreamID,
/// no unit here covers: it is never translated and never attached, and the
/// broker leaves it on its physical address, counted as unconfined.
pub(super) fn segment0(device: DeviceAddress) -> Result<(u8, u8, u8), IommuError> {
    match device.segment() {
        Some(0) => device.bdf().ok_or(IommuError::NoIommu),
        _ => Err(IommuError::NoIommu),
    }
}
