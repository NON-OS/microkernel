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

use nonos_libc::{mk_mmio_map, mk_mmio_unmap, MmioMapOut};

use super::labels::errno_label;
use super::rollback_modern::{rollback_one, rollback_two};
use super::state::{ModernGrant, RegisterGrant};
use crate::discover::Found;
use crate::setup::modern_caps::{self, Region};

const PAGE_SIZE: u64 = 4096;
const PAGE_MASK: u64 = PAGE_SIZE - 1;

pub fn map_modern(dev: Found, epoch: u64) -> Result<RegisterGrant, &'static str> {
    let caps = modern_caps::read(&dev, epoch)?;
    let common = map_region(dev, epoch, caps.common)?;
    let notify = match map_region(dev, epoch, caps.notify) {
        Ok(region) => region,
        Err(e) => return rollback_one(dev, common.out, e),
    };
    let device = match map_region(dev, epoch, caps.device) {
        Ok(region) => region,
        Err(e) => return rollback_two(dev, common.out, notify.out, e),
    };
    Ok(RegisterGrant::Modern(ModernGrant {
        common: common.out,
        common_offset: common.offset,
        notify: notify.out,
        notify_offset: notify.offset,
        notify_len: caps.notify.length as usize,
        notify_multiplier: caps.notify_multiplier as usize,
        device: device.out,
        device_offset: device.offset,
    }))
}

struct MappedRegion {
    out: MmioMapOut,
    offset: usize,
}

fn map_region(dev: Found, epoch: u64, region: Region) -> Result<MappedRegion, &'static str> {
    if region.length == 0 {
        return Err(errno_label(-22));
    }
    let base = region.offset as u64 & !PAGE_MASK;
    let offset = (region.offset as u64 - base) as usize;
    let length = ((offset as u64 + region.length as u64) + PAGE_MASK) & !PAGE_MASK;
    let mut out = MmioMapOut { user_va: 0, length: 0, grant_id: 0 };
    let rc = mk_mmio_map(dev.device_id, epoch, region.bar as u32, 0, base, length, &mut out);
    if rc < 0 {
        return Err(errno_label(rc));
    }
    // The kernel stops a mapping short of an MSI-X table and says so in the
    // length it returns. A region only partly mapped would fault on its
    // first access past the end, so it is all mapped or not used.
    if out.length < offset as u64 + region.length as u64 {
        let _ = mk_mmio_unmap(out.grant_id);
        return Err("virtio-gpu: mmio map short");
    }
    Ok(MappedRegion { out, offset })
}
