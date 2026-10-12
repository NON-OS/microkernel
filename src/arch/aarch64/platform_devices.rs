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

//! The device tree's platform devices: read while the tree is parsed at
//! boot, handed to the broker once it has a table and the kernel can map
//! registers, neither of which exists that early.

use spin::Once;

use crate::arch::fdt::find::platform::{self, PlatformDevice, PlatformKind, MAX_PLATFORM};
use crate::arch::fdt::{Fdt, FdtError};
use crate::hardware::broker::described::{self, BusWindow, Described, Interface};

static FOUND: Once<([PlatformDevice; MAX_PLATFORM], usize)> = Once::new();

/// Keep the tree's allowlisted devices; boot calls this once.
pub fn describe(fdt: &Fdt) -> Result<(), FdtError> {
    let mut all = [PlatformDevice::EMPTY; MAX_PLATFORM];
    let n = platform::find(fdt, &mut all)?;
    FOUND.call_once(|| (all, n));
    Ok(())
}

/// Publish them to the broker; returns how many it took.
pub fn publish() -> usize {
    let Some((all, n)) = FOUND.get() else {
        return 0;
    };
    all[..*n].iter().filter_map(described_from).filter_map(|d| described::register(&d)).count()
}

/// Whether `intid` is a platform device's SPI, and if so its trigger. The
/// bind path configures the line from this, never from the capsule.
pub fn trigger(intid: u32) -> Option<bool> {
    let (all, n) = FOUND.get()?;
    all[..*n].iter().find_map(|p| p.irq.filter(|irq| irq.intid == intid)).map(|irq| irq.edge)
}

/// GENET is described but not published: the broker has to stop a device's
/// DMA when its capsule lets go, and there is no public specification of
/// its registers to write that stop from.
fn described_from(p: &PlatformDevice) -> Option<Described> {
    let interface = match p.kind {
        PlatformKind::Sdhci => Interface::Sdhci,
        PlatformKind::Xhci => Interface::Xhci,
        PlatformKind::Ahci => Interface::Ahci,
        PlatformKind::GenetV5 => return None,
    };
    let dma = p.dma.map(|w| BusWindow { bus_base: w.bus_base, cpu_base: w.cpu_base, size: w.size });
    Some(Described {
        interface,
        windows: p.windows,
        window_count: p.window_count,
        irq: p.irq.map(|irq| (irq.intid, irq.edge)),
        dma,
    })
}
