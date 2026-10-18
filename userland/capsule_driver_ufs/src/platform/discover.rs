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

//! The first UFS host in the broker's device list that the table names.

use alloc::vec;

use nonos_libc::{mk_device_list, DeviceRecord, BAR_KIND_MMIO, BUS_KIND_PCI};

use super::hosts::{is_host, is_ufs_class};
use super::say::unlisted;

/// The broker lists every PCI function and ACPI device; a laptop has well
/// under a hundred.
const MAX_RECORDS: usize = 512;

#[derive(Clone, Copy)]
pub struct Found {
    pub device_id: u64,
    pub vendor: u16,
    pub device: u16,
    /// BAR0, the UFSHCI register space, rounded down to whole pages.
    pub bar0_len: u64,
}

pub fn find() -> Option<Found> {
    let total = mk_device_list(0, core::ptr::null_mut(), 0);
    let room = core::cmp::min(total.max(0) as usize, MAX_RECORDS);
    let mut buf = vec![DeviceRecord::empty(); room];
    let n = mk_device_list(0, buf.as_mut_ptr(), room as u64);
    let n = core::cmp::min(n.max(0) as usize, room);
    for r in buf[..n].iter().filter(|r| r.bus_kind == BUS_KIND_PCI) {
        if !is_ufs_class(r.pci_class, r.pci_subclass) {
            continue;
        }
        if !is_host(r.vendor, r.device) {
            unlisted(r.vendor, r.device);
            continue;
        }
        let bar = r.bars[0];
        if r.bar_count == 0 || bar.kind != BAR_KIND_MMIO || bar.size < 0x1000 {
            continue;
        }
        let bar0_len = bar.size & !0xFFF;
        return Some(Found {
            device_id: r.device_id,
            vendor: r.vendor,
            device: r.device,
            bar0_len,
        });
    }
    None
}
