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

use super::{iova::IOVA_BASE, reserved_rule::Held};
use crate::memory::iommu::{DeviceAddress, IommuDomain};

/// A capsule's domain and what it holds. Dropping the entry frees the domain.
pub(super) struct Capsule {
    pub pid: u32,
    pub domain: IommuDomain,
    pub devices: Vec<(u64, DeviceAddress)>,
    pub next_iova: u64,
    /// Device address space freed below `next_iova`, reused before it grows.
    pub holes: Vec<(u64, u64)>,
    /// Reserved regions mapped one to one for a device's sake (L60).
    pub reserved: Vec<Held>,
}

impl Capsule {
    /// Before its first grant: nothing mapped, the whole space ahead of it.
    pub(super) fn new(pid: u32, domain: IommuDomain) -> Self {
        let (devices, holes, reserved) = (Vec::new(), Vec::new(), Vec::new());
        Self { pid, domain, devices, next_iova: IOVA_BASE, holes, reserved }
    }
}

// Taken before the IOMMU's own lock, never inside it.
pub(super) static CAPSULES: Mutex<Vec<Capsule>> = Mutex::new(Vec::new());

/// The requester id a broker device's DMA carries, or `None` for one no
/// remapping unit sits in front of, such as an ACPI-enumerated controller. A
/// function behind an Intel VMD reaches the unit as the VMD, so several
/// devices can share one address.
pub(super) fn pci_address(device_id: u64) -> Option<DeviceAddress> {
    let handle = crate::hardware::broker::pci_index::lookup(device_id)?;
    let a = crate::drivers::pci::vmd::dma_requester(handle.address);
    Some(DeviceAddress::pci(a.bus, a.device, a.function))
}

pub(super) fn say(what: &[u8], pid: u32, device: DeviceAddress) {
    let serial = crate::sys::serial::print;
    serial(b"[VT-D] pid=");
    crate::sys::serial::print_dec(pid as u64);
    // Segment above the requester id, so segment 0 prints as it always has.
    match device {
        DeviceAddress::Pci { segment, rid } => {
            serial(b" device=");
            crate::sys::serial::print_hex((segment as u64) << 16 | rid as u64);
        }
        DeviceAddress::Stream(sid) => {
            serial(b" stream=");
            crate::sys::serial::print_hex(sid as u64);
        }
    }
    serial(b" ");
    crate::sys::serial::println(what);
}
