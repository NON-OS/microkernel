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

extern crate alloc;

use alloc::vec::Vec;
use spin::Mutex;

use crate::memory::iommu::{DeviceAddress, IommuDomain};

/// A capsule's domain and what it holds. Dropping the entry frees the domain.
pub(super) struct Capsule {
    pub pid: u32,
    pub domain: IommuDomain,
    pub devices: Vec<(u64, DeviceAddress)>,
    pub next_iova: u64,
}

// Taken before the IOMMU's own lock, never inside it.
pub(super) static CAPSULES: Mutex<Vec<Capsule>> = Mutex::new(Vec::new());

/// The PCI address of a broker device, or `None` for one no remapping unit
/// sits in front of, such as an ACPI-enumerated controller.
pub(super) fn pci_address(device_id: u64) -> Option<DeviceAddress> {
    let handle = crate::hardware::broker::pci_index::lookup(device_id)?;
    let a = handle.address;
    Some(DeviceAddress::pci(a.bus, a.device, a.function))
}

pub(super) fn say(what: &[u8], pid: u32, device: DeviceAddress) {
    let serial = crate::sys::serial::print;
    serial(b"[VT-D] pid=");
    crate::sys::serial::print_dec(pid as u64);
    serial(b" device=");
    crate::sys::serial::print_hex(device.as_u32() as u64);
    serial(b" ");
    crate::sys::serial::println(what);
}
