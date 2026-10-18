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

//! A host bridge's legacy INTx wiring, from its `interrupt-map`.

/// Routes kept from one map: a root bus has at most 32 slots of 4 pins,
/// but boards swizzle so every slot repeats the same four lines. More than
/// this is refused, not truncated.
pub const MAX_INTX: usize = 64;

/// One map entry, already decoded: requester bits and pin as the map
/// matches them, and the GIC SPI with its trigger.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct IntxRoute {
    pub child_hi: u32,
    pub pin: u32,
    pub intid: u32,
    pub edge: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntxMap {
    pub routes: [IntxRoute; MAX_INTX],
    pub count: usize,
    /// `interrupt-map-mask`, the address high cell and the pin.
    pub mask_hi: u32,
    pub mask_pin: u32,
    /// The bus the map speaks for: the bridge's first. A function behind a
    /// PCI bridge needs its pin swizzled through every bridge above it, so
    /// it is not routed from here.
    pub root_bus: u8,
}

impl IntxMap {
    /// The SPI and trigger for `pin` (1 to 4, INTA to INTD) of the function
    /// at `bus:device.function`, or `None` when the map does not wire it.
    pub fn route(&self, bus: u8, device: u8, function: u8, pin: u8) -> Option<(u32, bool)> {
        if bus != self.root_bus || !(1..=4).contains(&pin) || device > 31 || function > 7 {
            return None;
        }
        let hi = (u32::from(bus) << 16 | u32::from(device) << 11 | u32::from(function) << 8)
            & self.mask_hi;
        let pin = u32::from(pin) & self.mask_pin;
        self.routes[..self.count]
            .iter()
            .find(|r| r.child_hi == hi && r.pin == pin)
            .map(|r| (r.intid, r.edge))
    }
}
