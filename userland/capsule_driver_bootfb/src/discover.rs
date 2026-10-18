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

//! The boot framebuffer among the display-class devices the kernel lists,
//! read into `Listed` so `record::boot_fb` decides; nothing is claimed.

use nonos_libc::{mk_device_list, DeviceRecord};

use crate::record::{boot_fb, Listed, CLASS_DISPLAY};

/// Display controllers and firmware records together; a machine with
/// more than this lists the firmware one among the first all the same.
const MAX_DEVICES: usize = 64;

pub fn find() -> Option<Listed> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(CLASS_DISPLAY, buf.as_mut_ptr(), MAX_DEVICES as u64);
    let n = usize::try_from(n).ok()?.min(MAX_DEVICES);
    buf[..n].iter().map(listed).find(|l| boot_fb(l).is_some())
}

fn listed(r: &DeviceRecord) -> Listed {
    let b = r.bars[0];
    Listed {
        device_id: r.device_id,
        bus_kind: r.bus_kind,
        class: r.class,
        vendor: r.vendor,
        device: r.device,
        bar_count: r.bar_count,
        bar0: (b.kind, b.flags, b.base, b.size, b.aux),
    }
}
