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

use crate::arch::aarch64::firmware;
use crate::arch::fdt::find::platform::{PlatformDevice, PlatformKind};
use crate::hardware::broker::described::{BusWindow, Described, Interface};

/// GENET is described but not published: the broker has to stop a device's
/// DMA when its capsule lets go, and there is no public specification of
/// its registers to write that stop from.
pub(super) fn described_from(p: &PlatformDevice) -> Option<Described> {
    let interface = match p.kind {
        PlatformKind::Sdhci => Interface::Sdhci,
        PlatformKind::Xhci => Interface::Xhci,
        PlatformKind::Ahci => Interface::Ahci,
        PlatformKind::GenetV5 => return None,
    };
    let dma = p.dma.map(|w| BusWindow { bus_base: w.bus_base, cpu_base: w.cpu_base, size: w.size });
    // The Pi 4's eMMC2 is the only SDHCI on the allowlist, and its clock is
    // the firmware's to say.
    let clock_hz = match p.kind {
        PlatformKind::Sdhci => firmware::clock_rate(firmware::CLOCK_EMMC2).unwrap_or(0),
        _ => 0,
    };
    Some(Described {
        interface,
        windows: p.windows,
        window_count: p.window_count,
        clock_hz,
        irq: p.irq.map(|irq| (irq.intid, irq.edge)),
        dma,
    })
}
