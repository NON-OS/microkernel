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

//! Reading a virtio device's VIRTIO_F_ACCESS_PLATFORM before it is confined
//!: the rule is virtio_rule.rs; this reads the hardware it needs.

use super::virtio_read::{access_platform, FEATURE};
use super::virtio_rule::{admitted, common_cfg, is_modern_virtio};
use crate::drivers::pci::config::access::{read32, read8};
use crate::memory::addr::PhysAddr;
use crate::memory::mmio::{map_device_memory, unmap_mmio};

const PAGE: u64 = 4096;

/// Whether the device may be confined: true for anything not virtio.
pub(super) fn may_confine(device_id: u64) -> bool {
    let Some(rec) = crate::hardware::broker::list().into_iter().find(|r| r.device_id == device_id)
    else {
        return false;
    };
    let platform =
        is_modern_virtio(rec.vendor, rec.device).then(|| platform_bit(&rec)).flatten();
    admitted(rec.vendor, rec.device, platform)
}

/// Bit 33, read as device_feature word 1, bit 1; None when it cannot be read.
fn platform_bit(rec: &crate::hardware::broker::DeviceRecord) -> Option<bool> {
    let a = super::super::pci_index::lookup(rec.device_id)?.address;
    // Behind a VMD the segment 0 transport reaches another device: no read.
    if a.segment != 0 {
        return None;
    }
    let (bus, dev, func) = (a.bus, a.device, a.function);
    let byte = |off: u8| read8(bus, dev, func, u16::from(off)).ok();
    let word = |off: u8| read32(bus, dev, func, u16::from(off)).ok();
    let (bar, offset) = common_cfg(byte, word)?;
    let b =
        rec.bars.get(usize::from(bar)).filter(|b| b.kind == super::super::BarKind::Mmio as u8)?;
    let end = u64::from(offset).checked_add(FEATURE + 4)?;
    let at = b.base.checked_add(u64::from(offset)).filter(|_| end <= b.size)?;
    let page = at & !(PAGE - 1);
    let va = map_device_memory(PhysAddr::new(page), PAGE as usize).ok()?;
    let reg = |r: u64| (va.as_u64() + (at - page) + r) as *mut u32;
    // SAFETY: eK@nonos.systems - both registers lie inside the BAR (checked
    // against its size above) and inside the one uncached page just mapped,
    // which is unmapped before return and used by nothing else.
    let read = |r: u64| unsafe { core::ptr::read_volatile(reg(r)) };
    // SAFETY: as above.
    let write = |r: u64, v: u32| unsafe { core::ptr::write_volatile(reg(r), v) };
    let platform = access_platform(read, write);
    let _ = unmap_mmio(va);
    platform
}
