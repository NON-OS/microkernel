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

use super::iova;
use super::table::CAPSULES;
use crate::memory::addr::PhysAddr;
use crate::memory::iommu::IommuProtection;

/// The address a device is given for a grant, and whether it is an IOVA in
/// the capsule's domain. A device no unit confines gets the physical address,
/// and the grant is marked so its teardown knows there is nothing to unmap.
pub(in crate::hardware::broker) fn map(
    pid: u32,
    device_id: u64,
    phys: u64,
    length: u64,
) -> Option<(u64, bool)> {
    let mut all = CAPSULES.lock();
    let held = |c: &&mut super::table::Capsule| c.devices.iter().any(|(d, _)| *d == device_id);
    let Some(c) = all.iter_mut().filter(|c| c.pid == pid).find(held) else {
        return Some((phys, false));
    };
    let iova = iova::take(c, length)?;
    // SAFETY: eK@nonos.systems - `[phys, phys+length)` is a run the broker just
    // allocated and zeroed for this grant, and frees only after `unmap` below
    // returns true. `[iova, iova+length)` was taken fresh from this domain's
    // allocator, so nothing is mapped there. Both are page aligned: the broker
    // refuses a length that is not, and IOVA_BASE is.
    let mapped = unsafe {
        c.domain.map(iova, PhysAddr::new(phys), length as usize, IommuProtection::READ_WRITE)
    };
    if mapped.is_err() {
        iova::give_back(c, iova, length);
        return None;
    }
    Some((iova, true))
}

/// True once no device can reach the grant, which is when its frames may be
/// scrubbed and handed to someone else. False means they must not be.
pub(in crate::hardware::broker) fn unmap(pid: u32, iova: u64, length: u64, confined: bool) -> bool {
    if !confined {
        return true;
    }
    let mut all = CAPSULES.lock();
    let Some(c) = all.iter_mut().find(|c| c.pid == pid) else {
        // The capsule's last device was detached, which denied it this too.
        return true;
    };
    if c.domain.unmap(iova, length as usize).is_err() {
        return false;
    }
    iova::give_back(c, iova, length);
    true
}
