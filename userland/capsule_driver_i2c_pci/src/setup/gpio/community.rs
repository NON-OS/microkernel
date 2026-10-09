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

use nonos_libc::{
    mk_device_claim, mk_device_list, mk_device_release, mk_mmio_map, DeviceRecord, MmioMapOut,
    BUS_KIND_ACPI,
};

use crate::discover::CLASS_GPIO_CTRL;
use crate::regs::Regs;

const MAX_DEVICES: usize = 128;

/// Claim the GPIO controller record whose `_UID` is `uid`, with its claim
/// epoch. None when there is no such record or another capsule holds it.
pub(super) fn claim(uid: u16) -> Option<(DeviceRecord, u64)> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    let count = if n > 0 { (n as usize).min(MAX_DEVICES) } else { 0 };
    let rec = *buf[..count]
        .iter()
        .find(|r| r.bus_kind == BUS_KIND_ACPI && r.class == CLASS_GPIO_CTRL && r.device == uid)?;
    let epoch = mk_device_claim(rec.device_id);
    if epoch <= 0 {
        return None;
    }
    Some((rec, epoch as u64))
}

/// Map community window `bar` of a claimed controller, at its own length:
/// the AMD bank is 0x400 bytes at 0xFED81500, and the broker maps the page
/// around a sub-page window. The claim is released when the map fails.
pub(super) fn map(rec: &DeviceRecord, epoch: u64, bar: u8) -> Option<(Regs, u64)> {
    let size = rec.bars.get(usize::from(bar))?.size;
    let mut out = MmioMapOut { user_va: 0, length: 0, grant_id: 0 };
    if bar >= rec.bar_count
        || mk_mmio_map(rec.device_id, epoch, bar.into(), 0, 0, size, &mut out) < 0
    {
        let _ = mk_device_release(rec.device_id);
        return None;
    }
    Some((Regs::new(out.user_va), out.length))
}
