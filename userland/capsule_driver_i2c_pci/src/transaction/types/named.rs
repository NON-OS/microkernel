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

//! Which pad OP_ACPI_HID names. A bind made while the fitted pad slept
//! keeps every pad declared on the controller in reach (setup/sequence/
//! settle.rs). Each ask names the next address in reach, in turn, the bound
//! one first, so a fitted pad that is not the first declaration is named
//! within as many asks as there are pads in reach. Once a client has read a
//! pad's HID descriptor the reach is that pad alone (narrow.rs), and every
//! ask from then on names it.

use core::cell::Cell;

use super::reach::Reach;

impl Reach {
    /// The address the next OP_ACPI_HID names, with the HID descriptor
    /// register declared for it. `asks` counts the asks so far, this one
    /// included once it returns. None when nothing is in reach.
    pub fn named(&self, asks: &Cell<usize>) -> Option<(u8, u16)> {
        let ask = asks.replace(asks.get().wrapping_add(1));
        let addrs = self.addrs();
        let addr = *addrs.get(ask.checked_rem(addrs.len())?)?;
        Some((addr, self.desc_reg(addr)?))
    }
}
