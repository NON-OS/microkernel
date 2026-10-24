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

//! Validated config-space access.
//!
//! What belongs here is the checking and the counting. How an access reaches
//! the bus is `transport`'s problem, which is where the port pair and the ECAM
//! windows both live. An offset up to 0xFFF is a register; whether this
//! function's can be reached above 0xFF depends on its route.

use super::super::error::Result;
use super::super::types::PciAddress;
use super::{check::check, counters, transport};

/// Generate a checked read and write for one width.
macro_rules! checked {
    ($read:ident, $write:ident, $ty:ty, $width:expr) => {
        pub fn $read(bus: u8, device: u8, function: u8, offset: u16) -> Result<$ty> {
            check(PciAddress::new(bus, device, function), offset, $width)?;
            counters::read();
            transport::$read(bus, device, function, offset)
        }

        pub fn $write(bus: u8, device: u8, function: u8, offset: u16, value: $ty) -> Result<()> {
            check(PciAddress::new(bus, device, function), offset, $width)?;
            counters::wrote();
            transport::$write(bus, device, function, offset, value)
        }
    };
}

checked!(read8, write8, u8, 1);
checked!(read16, write16, u16, 2);
checked!(read32, write32, u32, 4);
