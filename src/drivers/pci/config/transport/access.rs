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

//! How a config-space access reaches the bus: the route for its (segment,
//! bus, offset), then that mechanism's load or store. Every function here is
//! in segment 0; the other segments arrive with the `PciAddress` accessor.

use super::super::super::error::{PciError, Result};
use super::route::{route, Route};
use super::{ecam, legacy};

/// Only a PC has the port pair.
const PORTS: bool = cfg!(target_arch = "x86_64");
const SEGMENT: u16 = 0;

fn pick(bus: u8, offset: u16) -> Route {
    route(ecam::lookup(SEGMENT, bus), PORTS, SEGMENT, offset)
}

fn unreachable(bus: u8, device: u8, function: u8, offset: u16) -> PciError {
    PciError::ConfigAccessFailed { bus, device, function, offset }
}

/// Generate a read and a write for one width. A mechanism that cannot name
/// the register reports it unreachable, the same as having no mechanism.
macro_rules! access {
    ($read:ident, $write:ident, $ty:ty) => {
        pub(in crate::drivers::pci::config) fn $read(
            bus: u8,
            device: u8,
            function: u8,
            offset: u16,
        ) -> Result<$ty> {
            match pick(bus, offset) {
                Route::Ecam(w) => ecam::$read(&w, bus, device, function, offset),
                Route::Legacy => legacy::$read(bus, device, function, offset),
                Route::Unreachable => None,
            }
            .ok_or(unreachable(bus, device, function, offset))
        }

        pub(in crate::drivers::pci::config) fn $write(
            bus: u8,
            device: u8,
            function: u8,
            offset: u16,
            value: $ty,
        ) -> Result<()> {
            match pick(bus, offset) {
                Route::Ecam(w) => ecam::$write(&w, bus, device, function, offset, value),
                Route::Legacy => legacy::$write(bus, device, function, offset, value),
                Route::Unreachable => None,
            }
            .ok_or(unreachable(bus, device, function, offset))
        }
    };
}

access!(read8, write8, u8);
access!(read16, write16, u16);
access!(read32, write32, u32);
