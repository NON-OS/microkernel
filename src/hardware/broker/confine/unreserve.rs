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

//! A device's reserved regions and ranges leaving its capsule's domain: its
//! RMRR regions (L60), and the ranges no attached device there names any
//! more (`held::unname`, C-41).

extern crate alloc;

use super::held::unname;
use super::reserved_rule::Held;
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

/// Forget `device_id`'s RMRR regions, unmapping each no other device holds.
pub(super) fn unmap_reserved(c: &mut Capsule, device_id: u64) {
    let mine: alloc::vec::Vec<Held> =
        c.rmrr.iter().copied().filter(|h| h.device == device_id).collect();
    c.rmrr.retain(|h| h.device != device_id);
    for h in mine {
        if !c.rmrr.iter().any(|o| o.base == h.base && o.end == h.end) {
            let _ = c.domain.unmap(h.base, (h.end - h.base) as usize);
        }
    }
}
