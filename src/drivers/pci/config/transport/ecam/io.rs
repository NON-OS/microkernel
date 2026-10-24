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

//! Loads and stores through a window. A byte store here is a byte store on
//! the bus, unlike the port pair's read-modify-write, so writing Command does
//! not write back Status, whose error bits clear when written with a one.

use super::window::EcamWindow;

/// Generate a read and a write for one access width. `None` when the window
/// does not reach the register, which the caller reports as unreachable.
macro_rules! accessor {
    ($read:ident, $write:ident, $ty:ty) => {
        pub(in crate::drivers::pci::config) fn $read(
            w: &EcamWindow,
            bus: u8,
            device: u8,
            function: u8,
            offset: u16,
        ) -> Option<$ty> {
            let addr = w.locate(bus, device, function, offset)?;
            // SAFETY: the address is inside a window its producer mapped as
            // device memory, and the caller checked the offset is aligned to
            // the width, which config space answers at any width.
            Some(unsafe { core::ptr::read_volatile(addr as *const $ty) })
        }

        pub(in crate::drivers::pci::config) fn $write(
            w: &EcamWindow,
            bus: u8,
            device: u8,
            function: u8,
            offset: u16,
            value: $ty,
        ) -> Option<()> {
            let addr = w.locate(bus, device, function, offset)?;
            // SAFETY: as for the read above.
            unsafe { core::ptr::write_volatile(addr as *mut $ty, value) };
            Some(())
        }
    };
}

accessor!(read8, write8, u8);
accessor!(read16, write16, u16);
accessor!(read32, write32, u32);
