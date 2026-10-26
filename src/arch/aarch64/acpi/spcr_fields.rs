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

//! SPCR's encoded fields (Microsoft's Serial Port Console Redirection
//! table specification, which ACPI 6.5 lists but does not define; recalled,
//! unverified): the configured baud rate byte, and the interface types an
//! Arm machine or a PC server uses (DBG2's serial subtypes are the same
//! numbers).

/// 3, 4, 6 and 7 name 9600, 19200, 57600 and 115200; 0 means the rate the
/// firmware left the port at; anything else is reserved.
pub(crate) fn baud(code: u8) -> Option<u32> {
    match code {
        3 => Some(9600),
        4 => Some(19_200),
        6 => Some(57_600),
        7 => Some(115_200),
        _ => None,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Interface {
    Ns16550,
    Ns16550Subset,
    Pl011,
    SbsaGeneric32,
    SbsaGeneric,
    /// A 16550 whose registers the GAS places, at its access width.
    Ns16550Gas,
    /// A type the parser does not name; its number as given.
    Other(u8),
}

pub(crate) fn interface(code: u8) -> Interface {
    match code {
        0x00 => Interface::Ns16550,
        0x01 => Interface::Ns16550Subset,
        0x03 => Interface::Pl011,
        0x0D => Interface::SbsaGeneric32,
        0x0E => Interface::SbsaGeneric,
        0x12 => Interface::Ns16550Gas,
        n => Interface::Other(n),
    }
}
