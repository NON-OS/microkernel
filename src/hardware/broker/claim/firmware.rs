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
//! A claim refused for the firmware's flash (firmware_flash.rs): the PCH's
//! flash controller found by where it sits, whatever its id, and any
//! device, PCI or platform, with a memory window inside the flash's.

use crate::hardware::broker::device::BAR_KIND_MMIO;
use crate::hardware::broker::firmware_flash::{is_flash_function, overlaps_flash};
use crate::hardware::broker::{pci_index, table};

pub(super) fn is_firmware(device_id: u64) -> bool {
    if let Some(h) = pci_index::lookup(device_id) {
        let a = h.address;
        if is_flash_function(a.segment, a.bus, a.device, a.function) {
            return true;
        }
    }
    table::list().iter().filter(|r| r.device_id == device_id).any(|r| {
        let bars = &r.bars[..(r.bar_count as usize).min(r.bars.len())];
        bars.iter().any(|b| b.kind == BAR_KIND_MMIO && overlaps_flash(b.base, b.size))
    })
}
