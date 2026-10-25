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

//! One IOMMU domain per driver capsule. A device a capsule claims leaves the
//! identity domain for the capsule's own, which maps nothing until `MkDmaMap`
//! grants a buffer, so the device reaches that capsule's grants and faults on
//! everything else. A device no unit confines is refused (decide.rs).

extern crate alloc;

use alloc::vec::Vec;
use spin::Mutex;

use super::held::Held;
use super::iova::IOVA_BASE;
use crate::memory::iommu::{DeviceAddress, IommuDomain};

/// A capsule's domain and what it holds. Dropping the entry frees the domain.
pub(super) struct Capsule {
    pub pid: u32,
    pub domain: IommuDomain,
    pub devices: Vec<(u64, DeviceAddress)>,
    pub next_iova: u64,
    /// Its devices' reserved ranges, mapped while a device naming them is
    /// attached; their IOVAs are never handed to a grant.
    pub reserved: Vec<Held>,
    /// It holds neither Dma nor Mmio (port_only.rs): its devices block no
    /// group but across a shared conventional bus.
    pub port_only: bool,
}

impl Capsule {
    pub(super) fn new(pid: u32, domain: IommuDomain, port_only: bool) -> Self {
        let (devices, reserved) = (Vec::new(), Vec::new());
        Self { pid, domain, devices, next_iova: IOVA_BASE, reserved, port_only }
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
