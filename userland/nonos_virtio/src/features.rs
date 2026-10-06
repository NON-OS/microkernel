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

//! Which feature bits a modern driver accepts.
//!
//! VERSION_1 is the modern transport itself: a device that does not offer
//! it cannot be driven this way, so the negotiation fails rather than guess.
//! ACCESS_PLATFORM is taken whenever it is offered. The broker hands a
//! confined device I/O virtual addresses, and only a device that negotiated
//! ACCESS_PLATFORM sends its DMA through the IOMMU that translates them;
//! one that did not would read and write those numbers as physical memory.
//! Of the device-type bits, only those the driver handles are taken, and
//! no other transport bit (event index, indirect descriptors, packed rings)
//! ever is: the rings these drivers lay out are the plain split rings.

use crate::error::VirtioError;

/// Bit 32: the device follows virtio 1.0.
pub const VIRTIO_F_VERSION_1: u64 = 1 << 32;
/// Bit 33: the device's DMA goes through the platform IOMMU.
pub const VIRTIO_F_ACCESS_PLATFORM: u64 = 1 << 33;
/// The bits the specification reserves for device types: 0 to 23 and 50 up.
pub const DEVICE_TYPE_BITS: u64 = ((1 << 24) - 1) | (u64::MAX << 50);

/// The features to write back, given what the device offers and the
/// device-type bits the driver handles.
pub fn negotiate(offered: u64, driver_handles: u64) -> Result<u64, VirtioError> {
    if offered & VIRTIO_F_VERSION_1 == 0 {
        return Err(VirtioError::NoVersion1);
    }
    let device = offered & driver_handles & DEVICE_TYPE_BITS;
    let platform = offered & VIRTIO_F_ACCESS_PLATFORM;
    Ok(device | VIRTIO_F_VERSION_1 | platform)
}
