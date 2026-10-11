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

//! MMIO phase: BAR0 and BAR1, one 4 KiB page each. A failure gives back
//! whatever was mapped and the claim.

use nonos_libc::{mk_device_release, mk_mmio_map, mk_mmio_unmap, MmioMapOut};

use crate::constants::pci::{PT_BAR, PT_BYTES, VD_BAR, VD_BYTES};
use crate::discover::Found;

fn one(dev: Found, epoch: u64, bar: u32, bytes: u64) -> Option<MmioMapOut> {
    let mut out = MmioMapOut { user_va: 0, length: 0, grant_id: 0 };
    let r = mk_mmio_map(dev.device_id, epoch, bar, 0, 0, bytes, &mut out);
    (r >= 0 && out.length >= bytes).then_some(out)
}

pub fn map(dev: Found, epoch: u64) -> Result<(MmioMapOut, MmioMapOut), &'static str> {
    let Some(pt) = one(dev, epoch, PT_BAR, PT_BYTES) else {
        let _ = mk_device_release(dev.device_id);
        return Err("mmio map failed (BAR0)");
    };
    let Some(vd) = one(dev, epoch, VD_BAR, VD_BYTES) else {
        let _ = mk_mmio_unmap(pt.grant_id);
        let _ = mk_device_release(dev.device_id);
        return Err("mmio map failed (BAR1)");
    };
    Ok((pt, vd))
}
