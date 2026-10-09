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

//! The console line said for each NVMe function seen.

use nonos_libc::DeviceRecord;

use crate::constants::NVME_BAR_INDEX;
use crate::log::{emit, Line};

/// One line per NVMe function seen. The broker hands out an opaque device
/// id, not the bus address, so that id names the function.
pub(super) fn say_seen(r: &DeviceRecord, servable: bool, cache: bool, kept: bool) {
    let mut line = Line::new();
    line.text(b"controller ")
        .hex_digits(r.vendor as u64, 4)
        .text(b":")
        .hex_digits(r.device as u64, 4)
        .text(b" device ")
        .dec(r.device_id)
        .text(b" BAR0 ")
        .dec(r.bars[NVME_BAR_INDEX as usize].size)
        .text(b" bytes");
    if !servable {
        line.text(b", no MMIO register BAR0 of 16 KiB, skipped");
    } else if !kept {
        line.text(b", past the controllers one instance tries, skipped");
    } else if cache {
        line.text(b", Optane cache module, tried last");
    }
    emit(&mut line);
}
