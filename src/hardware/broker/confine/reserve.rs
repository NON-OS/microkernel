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

//! A device's reserved ranges into its capsule's domain, before the device
//! is attached to it (the C-34 ruling): all of them or the claim is refused.

extern crate alloc;

use alloc::vec::Vec;

use super::held::{name, unheld};
use super::iova::IOVA_BASE;
use super::table::Capsule;
use crate::memory::addr::PhysAddr;
use crate::memory::iommu::reserved::admit;
use crate::memory::iommu::{DeviceAddress, IommuProtection};

/// Map `device`'s ranges into `c`'s domain, hold their IOVAs from the
/// allocator and record `device` as naming them; or why not, in which case
/// nothing new was left mapped and nothing recorded.
pub(super) fn reserve(c: &mut Capsule, device: DeviceAddress) -> Result<(), &'static [u8]> {
    let found = crate::arch::device_reserved_regions(device);
    if let Some(why) = found.unresolved {
        return Err(why);
    }
    // A range the domain already holds exactly, from another function behind
    // the same host bridge, is in place; anything else must be admitted.
    let ranges = unheld(&c.reserved, found.ranges());
    // Everything below the bump pointer may be granted: held as one span.
    // Its RMRR regions (L60) are held too, so no range lands on one.
    let mut held: Vec<(u64, u64)> = c.reserved_spans();
    if c.next_iova > IOVA_BASE {
        held.push((IOVA_BASE, c.next_iova));
    }
    admit(&ranges, &held, crate::arch::reserved_memory_regions()).map_err(|r| r.reason())?;
    for (i, range) in ranges.iter().enumerate() {
        // SAFETY: `[phys, phys + len)` lies in a reserved-memory region
        // (`admit`), which the frame allocator never hands out, and is what
        // firmware set aside for this device. `[iova, iova + len)` meets no
        // grant and no other reserved range in this domain, so nothing is
        // mapped there. Both are page aligned (`admit`).
        let mapped = unsafe {
            c.domain.map(
                range.iova,
                PhysAddr::new(range.phys),
                range.len as usize,
                IommuProtection::READ_WRITE,
            )
        };
        if mapped.is_err() {
            // One refused after its entries were written may have left some.
            let _ = c.domain.unmap(range.iova, range.len as usize);
            for done in &ranges[..i] {
                let _ = c.domain.unmap(done.iova, done.len as usize);
            }
            return Err(b"a reserved range could not be mapped");
        }
    }
    name(&mut c.reserved, found.ranges(), device);
    Ok(())
}
