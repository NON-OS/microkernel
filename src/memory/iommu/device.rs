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

/// A DMA requester as the remapping unit sees it, which is not always the
/// device itself: a function behind an Intel VMD reaches the unit as the
/// VMD, and on SMMUv3 a PCI requester becomes a StreamID through the device
/// tree's `iommu-map`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DeviceAddress {
    /// `rid` is `bus << 8 | device << 3 | function`.
    Pci { segment: u16, rid: u16 },
    /// A platform device's SMMU StreamID, from the device tree's `iommus`.
    Stream(u32),
}

impl DeviceAddress {
    /// A requester in segment 0, the only one most machines have.
    pub const fn pci(bus: u8, device: u8, function: u8) -> Self {
        Self::pci_in(0, bus, device, function)
    }

    /// Device and function are masked to their fields, so an out-of-range
    /// one cannot spill into the next bus and name another device's context.
    pub const fn pci_in(segment: u16, bus: u8, device: u8, function: u8) -> Self {
        let rid = (bus as u16) << 8 | (device as u16 & 0x1F) << 3 | (function as u16 & 0x7);
        Self::Pci { segment, rid }
    }

    pub const fn stream(sid: u32) -> Self {
        Self::Stream(sid)
    }

    pub const fn segment(&self) -> Option<u16> {
        match *self {
            Self::Pci { segment, .. } => Some(segment),
            Self::Stream(_) => None,
        }
    }

    pub const fn rid(&self) -> Option<u16> {
        match *self {
            Self::Pci { rid, .. } => Some(rid),
            Self::Stream(_) => None,
        }
    }

    /// A PCI requester's bus, device and function.
    pub const fn bdf(&self) -> Option<(u8, u8, u8)> {
        match *self {
            Self::Pci { rid, .. } => {
                Some(((rid >> 8) as u8, ((rid >> 3) & 0x1F) as u8, (rid & 0x7) as u8))
            }
            Self::Stream(_) => None,
        }
    }
}
