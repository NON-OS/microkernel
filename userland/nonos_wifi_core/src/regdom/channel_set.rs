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

//! A set of legacy channel numbers (1 to 255), the shape a Country element's
//! channel triplets describe.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ChannelSet([u64; 4]);

impl ChannelSet {
    pub const EMPTY: Self = Self([0; 4]);

    pub fn insert(&mut self, channel: u8) {
        self.0[(channel >> 6) as usize] |= 1 << (channel & 63);
    }

    pub const fn contains(&self, channel: u8) -> bool {
        self.0[(channel >> 6) as usize] & (1 << (channel & 63)) != 0
    }

    pub fn is_empty(&self) -> bool {
        self.0.iter().all(|w| *w == 0)
    }
}
