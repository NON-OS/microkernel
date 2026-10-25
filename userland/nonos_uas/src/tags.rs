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
//! Tags in use. Each command in flight has its own, from 1; at
//! SuperSpeed the tag is the stream its data and status move on, so the
//! device's stream count bounds them, and one alone without streams.

#[derive(Clone, Copy, Debug)]
pub struct Tags {
    /// Bit n set: tag n + 1 in use. 32 commands in flight at most.
    used: u32,
    most: u8,
}

impl Tags {
    /// `most` tags, 1 to 32.
    pub fn new(most: u8) -> Tags {
        Tags { used: 0, most: most.clamp(1, 32) }
    }

    pub fn take(&mut self) -> Option<u16> {
        let free = (0..self.most).find(|&i| self.used & (1 << i) == 0)?;
        self.used |= 1 << free;
        Some(free as u16 + 1)
    }

    pub fn give(&mut self, tag: u16) {
        if (1..=self.most as u16).contains(&tag) {
            self.used &= !(1 << (tag - 1));
        }
    }
}
