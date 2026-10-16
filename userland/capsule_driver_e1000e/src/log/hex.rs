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

//! Numbers in a line: device IDs, in lowercase hex. Station addresses are
//! never written to the log, so there is no way here to write one.

use super::line::Line;

const HEX: &[u8; 16] = b"0123456789abcdef";

impl Line {
    /// `v` as four lowercase hex digits, the way lspci prints a device ID.
    pub fn hex16(&mut self, v: u16) -> &mut Self {
        self.hex8((v >> 8) as u8);
        self.hex8(v as u8)
    }

    fn hex8(&mut self, b: u8) -> &mut Self {
        self.byte(HEX[(b >> 4) as usize]);
        self.byte(HEX[(b & 0xF) as usize]);
        self
    }
}
