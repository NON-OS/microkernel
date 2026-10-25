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
//! which is how every claim stood before. The device tree's ranges are not
//! these: they go in all or nothing (`reserve`, C-40).

use super::reserved_rule::{verdict, Held, Verdict};
use super::say::say;
use super::table::Capsule;
use crate::memory::addr::PhysAddr;
use crate::memory::iommu::{DeviceAddress, IommuProtection};

/// `others`: every region another capsule's domain holds, never shared (L63).
pub(super) fn map_reserved(
    c: &mut Capsule,
    device_id: u64,
    address: DeviceAddress,
    others: &[(u64, u64)],
) {
    let (Some(segment), Some(bdf)) = (address.segment(), address.bdf()) else { return };
    let found = crate::arch::device_rmrr_regions(segment, bdf);
    if found.unresolved {
        say(b"an RMRR that may name it could not be resolved; not mapped");
    }
    if found.shared {
        say(b"an RMRR it shares with other devices; not mapped");
    }
    for &(base, end) in found.ranges.iter() {
        let ram = crate::memory::phys::touches_usable(base, end);
        let foreign = others.iter().any(|&(b, e)| b < end && base < e);
        match verdict(base, end, c.next_iova, &c.rmrr, ram || foreign) {
            Verdict::Refuse(why) => say(why),
            Verdict::Held => c.rmrr.push(Held { base, end, device: device_id }),
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
                    c.rmrr.push(Held { base, end, device: device_id });
                    say(b"its RMRR mapped one to one");
                } else {
                    let _ = c.domain.unmap(base, len);
                    say(b"its RMRR would not map; left out");
                }
            }
        }
    }
}
