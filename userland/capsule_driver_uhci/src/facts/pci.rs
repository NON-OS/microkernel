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

//! The PCI side: the class code and the legacy-support register.

/// Serial bus, USB, programming interface UHCI.
pub const CLASS_SERIAL: u8 = 0x0C;
pub const SUBCLASS_USB: u8 = 0x03;
pub const PROGIF_UHCI: u8 = 0x00;
/// The I/O registers are BAR4.
pub const IO_BAR: u32 = 4;

/// LEGSUP, config 0xC0, 16 bits: firmware's keyboard emulation trap and
/// SMI enables in 7:0, their status in 11:8 and 15 (each cleared by writing
/// it), and PIRQD routing in 13 (the Intel PIIX4 and ICH datasheets, as
/// recalled, unverified). A polled driver wants every trap and SMI off,
/// every status cleared, and no interrupt routed.
pub const LEGSUP: u32 = 0xC0;
pub const LEGSUP_CLEAR: u16 = 0x8F00;
/// Bits 12 and 14, which the driver has no business with, written as read.
pub const LEGSUP_KEEP: u16 = 0x5000;
