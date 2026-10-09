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

//! The register values a host reports and its specification version.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Caps {
    /// Capabilities, 0x40.
    pub caps: u32,
    /// Capabilities, 0x44.
    pub caps1: u32,
    /// Host Controller Version, 0xFE: spec version in 7:0, vendor in 15:8.
    pub version: u16,
}

impl Caps {
    /// The Specification Version Number: 0 = 1.00, 1 = 2.00, 2 = 3.00,
    /// 3 = 4.00, 4 = 4.10, 5 = 4.20.
    pub const fn spec(&self) -> u8 {
        self.version as u8
    }
}
