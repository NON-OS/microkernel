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

//! Reading the broker's device list and keeping the eMMC candidates.

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::{mk_device_list, DeviceRecord, BUS_KIND_PCI};

use super::super::super::pci::classify;
use super::Found;

/// Ask for at most this many device records. The broker lists every PCI
/// function and ACPI device; a laptop has well under a hundred.
const MAX_RECORDS: usize = 512;

/// The candidates in the order they are tried: known Intel eMMC hosts, then
/// other SDHCI hosts (each served only if its slot is embedded), each group
/// in device list order. SD card readers and SDIO hosts Intel names are
/// left out. The broker files class 0x08 under OTHER, so the whole list is
/// read (class 0) and matched on the PCI class codes.
pub fn discover() -> Vec<Found> {
    let total = mk_device_list(0, core::ptr::null_mut(), 0);
    if total <= 0 {
        return Vec::new();
    }
    let room = core::cmp::min(total as usize, MAX_RECORDS);
    let mut buf = vec![DeviceRecord::empty(); room];
    let n = mk_device_list(0, buf.as_mut_ptr(), room as u64);
    if n <= 0 {
        return Vec::new();
    }
    let mut found: Vec<Found> = buf[..core::cmp::min(n as usize, room)]
        .iter()
        .filter(|r| r.bus_kind == BUS_KIND_PCI)
        .filter_map(|r| {
            classify(r.vendor, r.device, r.pci_class, r.pci_subclass, r.pci_progif).map(|kind| {
                Found {
                    device_id: r.device_id,
                    vendor: r.vendor,
                    device: r.device,
                    kind,
                    bars: r.bars,
                    bar_count: r.bar_count,
                }
            })
        })
        .collect();
    // Stable: device list order is kept within each kind.
    found.sort_by_key(|f| f.kind);
    found
}
