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

//! Which capsule's domain holds a device now, read and never changed: for
//! the posture lines that say whether a device firmware asked to be kept
//! secured is confined at this moment.

use super::table::CAPSULES;
use crate::memory::iommu::DeviceAddress;

/// The pid whose domain holds `address`, None when no domain does.
pub(in crate::hardware::broker) fn held_by(address: DeviceAddress) -> Option<u32> {
    let all = CAPSULES.lock();
    all.iter().find(|c| c.devices.iter().any(|(_, a)| *a == address)).map(|c| c.pid)
}
