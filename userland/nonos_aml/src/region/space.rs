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

//! Region spaces, section 19.6.100's RegionSpace values.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Space(pub u8);

impl Space {
    pub const SYSTEM_MEMORY: Space = Space(0);
    pub const SYSTEM_IO: Space = Space(1);
    pub const PCI_CONFIG: Space = Space(2);
    pub const EMBEDDED_CONTROL: Space = Space(3);
    pub const SMBUS: Space = Space(4);
    pub const SYSTEM_CMOS: Space = Space(5);
    pub const PCI_BAR_TARGET: Space = Space(6);
    pub const IPMI: Space = Space(7);
    pub const GPIO: Space = Space(8);
    pub const GENERIC_SERIAL_BUS: Space = Space(9);
    pub const PCC: Space = Space(0x0A);

    /// Spaces whose fields move a buffer through the handler as one
    /// transaction (section 5.5.2.4.5 and on), rather than registers read
    /// and written by width.
    pub fn is_transactional(self) -> bool {
        matches!(self.0, 4 | 7 | 9 | 0x0A)
    }

    /// Spaces whose registers are bytes: the embedded controller's
    /// (section 12.3) and the CMOS's, reached through an index port.
    pub fn is_byte_wide(self) -> bool {
        matches!(self.0, 3 | 5)
    }

    /// A bit for the set of spaces a handler has been connected for; the
    /// OEM spaces 0x80-0xFF share the last.
    pub fn bit(self) -> u32 {
        1 << (self.0 as u32).min(31)
    }
}
