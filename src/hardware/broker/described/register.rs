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

use crate::hardware::broker::table::register_platform_device;
use crate::memory::addr::PhysAddr;

use super::interface::Interface;
use super::iommus::DescribedIommus;
use super::state::{Entry, DESCRIBED};
use super::window::BusWindow;

/// One device as its firmware description gives it, in CPU terms.
#[derive(Debug, Clone, Copy)]
pub struct Described {
    pub interface: Interface,
    pub windows: [(u64, u64); 4],
    pub window_count: usize,
    /// The controller's base clock in Hz when firmware states it, else 0;
    /// published as bar 0's `aux`, where an SDHCI with no clock in its
    /// capabilities register finds it.
    pub clock_hz: u32,
    /// The SPI and whether it is edge triggered.
    pub irq: Option<(u32, bool)>,
    /// `None`: no bus maps its DMA, so no grant is ever given for it.
    pub dma: Option<BusWindow>,
    /// `dma-coherent`: its DMA snoops the CPU's caches.
    pub coherent: bool,
    /// The IOMMU and streams its DMA goes through, if any.
    pub iommus: DescribedIommus,
}

/// Every stop reads registers in the first 4 KiB.
const STOP_WINDOW: u64 = 0x1000;

/// Publish `d` and return its device id. A device with no registers is not
/// one a capsule can drive, and is not published.
pub fn register(d: &Described) -> Option<u64> {
    let (base, size) = *d.windows[..d.window_count].first()?;
    let span = size.min(STOP_WINDOW) as usize;
    let regs = crate::memory::mmio::map_device_memory(PhysAddr::new(base), span).ok();
    if regs.is_none() {
        crate::sys::serial::println(b"[BROKER] a described device's registers did not map");
        return None;
    }
    let device_id = register_platform_device(super::record::record(d));
    DESCRIBED.write().push(Entry {
        device_id,
        interface: d.interface,
        regs: regs.map(|va| va.as_u64()),
        dma: d.dma,
        iommus: d.iommus,
    });
    Some(device_id)
}
