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
use super::map_fail::{after_refusal, MapFail};
use super::table::CAPSULES;
use crate::memory::addr::PhysAddr;
use crate::memory::iommu::{IommuError, IommuProtection};

/// Why a grant got no device address: no domain confines the device
/// (decide.rs), or the domain would not map it.
pub(in crate::hardware::broker) enum NoAddress {
    Unconfined,
    /// Nothing of the grant is mapped: its frames may be freed.
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
    access: IommuProtection,
) -> Result<(u64, bool), NoAddress> {
    let mut all = CAPSULES.lock();
    let held = |c: &&mut super::table::Capsule| c.devices.iter().any(|(d, _)| *d == device_id);
    let Some(c) = all.iter_mut().filter(|c| c.pid == pid).find(held) else {
        let physical = super::open::physical_dma();
        return if physical { Ok((phys, false)) } else { Err(NoAddress::Unconfined) };
    };
    let iova = iova::take(c, length).ok_or(NoAddress::Failed)?;
    // SAFETY: eK@nonos.systems - `[phys, phys+length)` is a zeroed run the broker
    // frees only after `unmap` returns true or a refused map is known to have
    // left it unmapped; or, with `DOORBELL`, an MSI doorbell page whose one
    // register takes a message and reads as zero. `[iova, iova+length)` is fresh
    // from this domain's allocator; both are page aligned.
    match unsafe { c.domain.map(iova, PhysAddr::new(phys), length as usize, access) } {
        Ok(()) => Ok((iova, true)),
        // Entries written but not confirmed, or a map the unit may still
        // hold: the frames go back only if an unmap takes the range away,
        // and a range none took keeps its IOVA too.
        Err(e) => {
            let unmap = || c.domain.unmap(iova, length as usize).is_ok();
            let fail = match e {
                IommuError::Stranded => MapFail::Pinned,
                e => after_refusal(e == IommuError::Uncommitted, unmap),
            };
            if fail == MapFail::Pinned {
                return Err(NoAddress::Stranded);
            }
            iova::give_back(c, iova, length);
            Err(NoAddress::Failed)
        }
    }
}
