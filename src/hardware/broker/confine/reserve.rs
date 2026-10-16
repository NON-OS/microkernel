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

//! A claimed device's reserved regions in its capsule's domain (L60). Each
//! RMRR that names the device is mapped one to one before the device enters
//! the domain, so the DMA firmware said it keeps doing (integrated graphics'
//! stolen memory, USB legacy buffers) still lands. A region that fails a
//! check is left out and said: mapping too little only denies the device,
//! which is how every claim stood before.

extern crate alloc;

use super::reserved_rule::{verdict, Held, Verdict};
use super::table::{say, Capsule};
use crate::memory::addr::PhysAddr;
use crate::memory::iommu::{DeviceAddress, IommuProtection};

pub(super) fn map_reserved(c: &mut Capsule, pid: u32, device_id: u64, address: DeviceAddress) {
    let (Some(segment), Some(bdf)) = (address.segment(), address.bdf()) else { return };
    let found = crate::arch::device_reserved_regions(segment, bdf);
    if found.1 {
        say(b"an RMRR that may name it could not be resolved; not mapped", pid, address);
    }
    for &(base, end) in found.0.iter() {
        let ram = crate::memory::phys::touches_usable(base, end);
        match verdict(base, end, c.next_iova, &c.reserved, ram) {
            Verdict::Refuse(why) => say(why, pid, address),
            Verdict::Held => c.reserved.push(Held { base, end, device: device_id }),
            Verdict::Map => {
                let len = (end - base) as usize;
                // SAFETY: eK@nonos.systems - `[base, end)` is a region firmware
                // reserved for this device's DMA, outside every usable RAM run
                // the allocator was given, so no frame of it is ever handed
                // out; its IOVA overlaps no grant and no other region here.
                let map = |c: &Capsule| unsafe {
                    c.domain.map(base, PhysAddr::new(base), len, IommuProtection::READ_WRITE)
                };
                if map(c).is_ok() {
                    c.reserved.push(Held { base, end, device: device_id });
                    say(b"its RMRR mapped one to one", pid, address);
                } else {
                    let _ = c.domain.unmap(base, len);
                    say(b"its RMRR would not map; left out", pid, address);
                }
            }
        }
    }
}

/// Forget `device_id`'s regions, unmapping each no other device holds.
pub(super) fn unmap_reserved(c: &mut Capsule, device_id: u64) {
    let mine: alloc::vec::Vec<Held> =
        c.reserved.iter().copied().filter(|h| h.device == device_id).collect();
    c.reserved.retain(|h| h.device != device_id);
    for h in mine {
        if !c.reserved.iter().any(|o| o.base == h.base && o.end == h.end) {
            let _ = c.domain.unmap(h.base, (h.end - h.base) as usize);
        }
    }
}
