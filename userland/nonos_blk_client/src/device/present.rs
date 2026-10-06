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

//! The storage controllers on the bus, from the kernel's device table.
//! Every class is asked for, not only block: some VMD functions report a
//! system peripheral class, and they are the ones that hide the disks.
//! A capsule without `DeviceEnum` is refused the table and sees none.

use alloc::vec::Vec;

use nonos_libc::{mk_device_list, DeviceRecord, BUS_KIND_PCI};

use crate::driver::{classify, Controller};

/* A machine with more PCI functions than this has its first ones read. */
const MAX_RECORDS: usize = 256;
const ANY_CLASS: u32 = 0;

pub fn controllers() -> Vec<Controller> {
    let total = mk_device_list(ANY_CLASS, core::ptr::null_mut(), 0);
    if total <= 0 {
        return Vec::new();
    }
    let mut buf = alloc::vec![DeviceRecord::empty(); (total as usize).min(MAX_RECORDS)];
    let n = mk_device_list(ANY_CLASS, buf.as_mut_ptr(), buf.len() as u64);
    if n <= 0 {
        return Vec::new();
    }
    buf.truncate((n as usize).min(MAX_RECORDS));
    buf.iter()
        .filter(|r| r.bus_kind == BUS_KIND_PCI)
        .filter_map(|r| classify(r.vendor, r.device, r.pci_class, r.pci_subclass, r.pci_progif))
        .collect()
}
