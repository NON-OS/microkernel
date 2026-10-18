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

//! Legacy INTx on aarch64: the device tree's `interrupt-map` is the only
//! word on which SPI a PCI function's pin raises. Config space's interrupt
//! line is firmware's note on x86 and never written here, so trusting it
//! would let a capsule bind whatever SPI that byte happened to name.

use spin::Once;

use crate::arch::fdt::find::pci::IntxMap;

static MAP: Once<IntxMap> = Once::new();

/// Record the map; boot calls this once, before PCI is enumerated.
pub fn describe(map: IntxMap) {
    MAP.call_once(|| map);
}

/// The SPI `pin` of `bus:device.function` raises, with its trigger, or
/// `None` when the tree does not wire it.
pub fn route(bus: u8, device: u8, function: u8, pin: u8) -> Option<(u32, bool)> {
    MAP.get()?.route(bus, device, function, pin)
}

/// Whether `intid` is an INTx SPI the map names, and if so its trigger.
/// The bind path configures the line from this, never from the capsule.
pub fn trigger(intid: u32) -> Option<bool> {
    let map = MAP.get()?;
    map.routes[..map.count].iter().find(|r| r.intid == intid).map(|r| r.edge)
}
