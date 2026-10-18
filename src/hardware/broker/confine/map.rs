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
use crate::memory::iommu::{IommuError, IommuProtection};

/// Why a grant got no device address: no domain confines the device
/// (decide.rs), or the domain would not map it.
pub(in crate::hardware::broker) enum NoAddress {
    Unconfined,
    Failed,
    /// The failed map may still be reachable: its frames are never reused.
    Stranded,
}

/// A grant's device address, and whether it is an IOVA in the capsule's
/// domain. Under the development opt-in alone, an unconfined device gets its
/// physical address, marked so its teardown knows there is nothing to unmap.
pub(in crate::hardware::broker) fn map(
    pid: u32,
    device_id: u64,
    phys: u64,
    length: u64,
) -> Result<(u64, bool), NoAddress> {
    let mut all = CAPSULES.lock();
    let held = |c: &&mut super::table::Capsule| c.devices.iter().any(|(d, _)| *d == device_id);
    let Some(c) = all.iter_mut().filter(|c| c.pid == pid).find(held) else {
        return if super::decide::physical_dma() {
            Ok((phys, false))
        } else {
            Err(NoAddress::Unconfined)
        };
    };
    let iova = iova::take(c, length).ok_or(NoAddress::Failed)?;
    // SAFETY: eK@nonos.systems - `[phys, phys+length)` is a run the broker just
    // allocated and zeroed for this grant, and frees only after `unmap` below
    // returns true. `[iova, iova+length)` was taken fresh from this domain's
    // allocator, so nothing is mapped there. Both are page aligned: the broker
    // refuses a length that is not, and IOVA_BASE is.
    let mapped = unsafe {
        c.domain.map(iova, PhysAddr::new(phys), length as usize, IommuProtection::READ_WRITE)
    };
    match mapped {
        Ok(()) => {}
        // The IOVA stays taken as well: the unit may still translate it.
        Err(IommuError::Stranded) => return Err(NoAddress::Stranded),
        Err(_) => {
            iova::give_back(c, iova, length);
            return Err(NoAddress::Failed);
        }
    }
    Ok((iova, true))
}
