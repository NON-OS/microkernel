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
    /// The PCI Express capability's offset, when the function has one.
    pub pcie_cap: Option<u16>,
}
