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

//! The PCI side: the class code and the register BAR.

/// Serial bus, USB, programming interface EHCI.
pub const CLASS_SERIAL: u8 = 0x0C;
pub const SUBCLASS_USB: u8 = 0x03;
pub const PROGIF_EHCI: u8 = 0x20;
/// The capability and operational registers are memory BAR0.
pub const MMIO_BAR: u32 = 0;
/// The capability registers and room for the operational ones.
pub const MMIO_MIN: u64 = 0x100;
