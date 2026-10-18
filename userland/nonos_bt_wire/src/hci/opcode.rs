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

//! A command opcode (Vol 4, Part E, 5.4.1): a 6-bit OGF over a 10-bit OCF,
//! sent little-endian. The opcodes named are those an LE-only host uses to
//! bring a controller up and run a connection; each cites its command.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Opcode(pub u16);

impl Opcode {
    pub const fn new(ogf: u8, ocf: u16) -> Self {
        Self(((ogf as u16 & 0x3f) << 10) | (ocf & 0x3ff))
    }

    pub const fn ogf(self) -> u8 {
        (self.0 >> 10) as u8
    }

    pub const fn ocf(self) -> u16 {
        self.0 & 0x3ff
    }
}

/// Vol 4, Part E, 7.1.6.
pub const DISCONNECT: Opcode = Opcode::new(0x01, 0x0006);
/// Vol 4, Part E, 7.3.1.
pub const SET_EVENT_MASK: Opcode = Opcode::new(0x03, 0x0001);
/// Vol 4, Part E, 7.3.2.
pub const RESET: Opcode = Opcode::new(0x03, 0x0003);
/// Vol 4, Part E, 7.4.1.
pub const READ_LOCAL_VERSION: Opcode = Opcode::new(0x04, 0x0001);
/// Vol 4, Part E, 7.4.3.
pub const READ_LOCAL_FEATURES: Opcode = Opcode::new(0x04, 0x0003);
/// Vol 4, Part E, 7.4.5.
pub const READ_BUFFER_SIZE: Opcode = Opcode::new(0x04, 0x0005);
/// Vol 4, Part E, 7.4.6.
pub const READ_BD_ADDR: Opcode = Opcode::new(0x04, 0x0009);
/// Vol 4, Part E, 7.8.1.
pub const LE_SET_EVENT_MASK: Opcode = Opcode::new(0x08, 0x0001);
/// Vol 4, Part E, 7.8.2.
pub const LE_READ_BUFFER_SIZE: Opcode = Opcode::new(0x08, 0x0002);
/// Vol 4, Part E, 7.8.3.
pub const LE_READ_LOCAL_FEATURES: Opcode = Opcode::new(0x08, 0x0003);
/// Vol 4, Part E, 7.8.4.
pub const LE_SET_RANDOM_ADDRESS: Opcode = Opcode::new(0x08, 0x0005);
/// Vol 4, Part E, 7.8.10.
pub const LE_SET_SCAN_PARAMETERS: Opcode = Opcode::new(0x08, 0x000b);
/// Vol 4, Part E, 7.8.11.
pub const LE_SET_SCAN_ENABLE: Opcode = Opcode::new(0x08, 0x000c);
/// Vol 4, Part E, 7.8.12.
pub const LE_CREATE_CONNECTION: Opcode = Opcode::new(0x08, 0x000d);
/// Vol 4, Part E, 7.8.13.
pub const LE_CREATE_CONNECTION_CANCEL: Opcode = Opcode::new(0x08, 0x000e);
/// Vol 4, Part E, 7.8.18.
pub const LE_CONNECTION_UPDATE: Opcode = Opcode::new(0x08, 0x0013);
/// Vol 4, Part E, 7.8.24.
pub const LE_ENABLE_ENCRYPTION: Opcode = Opcode::new(0x08, 0x0019);
/// Vol 4, Part E, 7.8.25.
pub const LE_LTK_REQUEST_REPLY: Opcode = Opcode::new(0x08, 0x001a);
/// Vol 4, Part E, 7.8.26.
pub const LE_LTK_REQUEST_NEGATIVE_REPLY: Opcode = Opcode::new(0x08, 0x001b);
