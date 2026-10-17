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

//! A device's reserved ranges out of its capsule's domain, once no attached
//! device there names them (`held::unname`).

use super::held::unname;
use super::table::Capsule;
use crate::memory::iommu::DeviceAddress;

/// `device` is out of `c`'s domain, or never made it in: unmap every range
/// it was the last to name, so the capsule's other devices cannot reach
/// them either. Called under the capsule lock, after the device's detach.
pub(super) fn release(c: &mut Capsule, device: DeviceAddress) {
    for range in unname(&mut c.reserved, device) {
        let _ = c.domain.unmap(range.iova, range.len as usize);
    }
}
