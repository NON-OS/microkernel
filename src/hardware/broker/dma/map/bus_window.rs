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

//! A described device's DMA window: the frames it may be given and the bus
//! address it is told for them. The Pi 4's eMMC2 reaches only RAM's first
//! gigabyte, and at 0xc000_0000 on its bus; a PCI function, or a device an
//! SMMU confines, keeps the address `confine` gave it.

use crate::hardware::broker::described;
use crate::hardware::broker::dma::flags::DMA_MAP_DMA32;
use crate::hardware::broker::dma::types::DmaMapError;

/// The flags to take frames with. A window that ends below 4 GiB takes them
/// from where the low pool is, the lowest RAM there is. A device no bus
/// maps DMA for is given nothing.
pub(super) fn alloc_flags(device_id: u64, flags: u32) -> Result<u32, DmaMapError> {
    match described::dma_window(device_id) {
        None => Ok(flags),
        Some(None) => Err(DmaMapError::MapFailed),
        Some(Some(w)) if w.ends_below_4g() => Ok(flags | DMA_MAP_DMA32),
        Some(Some(_)) => Ok(flags),
    }
}

/// The address to hand the device for `[phys, phys + len)`: `given` unless
/// the device is a described one that no SMMU confines, whose bus moves it.
/// `None` when the frames landed outside its window.
pub(super) fn device_address(
    device_id: u64,
    phys: u64,
    len: u64,
    given: u64,
    confined: bool,
) -> Option<u64> {
    match described::dma_window(device_id) {
        Some(Some(w)) if !confined => w.bus_address(phys, len),
        Some(None) => None,
        _ => Some(given),
    }
}
