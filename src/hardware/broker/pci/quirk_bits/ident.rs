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

//! The claimed function as the vendor-bit rules see it.

/// What the broker knows of the claimed function.
#[derive(Debug, Clone, Copy)]
pub struct Ident {
    pub vendor: u16,
    pub class: u8,
    pub subclass: u8,
    /// The programming interface: which host a USB controller is.
    pub prog_if: u8,
    /// For an EHCI function, the config bytes 2, 4 and 6 below the written
    /// offset (0xFF where none is read, and all 0xFF inside a capability on
    /// the standard list): whether the offset falls in its legacy support
    /// capability, whose first byte is its id, 01h (legsup.rs).
    pub before: [u8; 3],
    /// The PCI Express capability's offset, when the function has one.
    pub pcie_cap: Option<u16>,
}
