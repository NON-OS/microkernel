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

//! Numbers in a line: device IDs and MAC addresses, in lowercase hex.

use super::line::Line;

const HEX: &[u8; 16] = b"0123456789abcdef";

impl Line {
    /// `v` as four lowercase hex digits, the way lspci prints a device ID.
    pub fn hex16(&mut self, v: u16) -> &mut Self {
        self.hex8((v >> 8) as u8);
        self.hex8(v as u8)
    }

    pub fn mac(&mut self, mac: &[u8; 6]) -> &mut Self {
        for (i, b) in mac.iter().enumerate() {
            self.text(if i > 0 { ":" } else { "" }).hex8(*b);
        }
        self
    }

    fn hex8(&mut self, b: u8) -> &mut Self {
        self.byte(HEX[(b >> 4) as usize]);
        self.byte(HEX[(b & 0xF) as usize]);
        self
    }
}
