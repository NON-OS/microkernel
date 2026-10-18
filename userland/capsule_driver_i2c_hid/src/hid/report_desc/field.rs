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

//! One field of a report as the descriptor walk located it: its first bit
//! from the start of the report body (after the report ID byte, if any),
//! its width, and its logical range.

#[derive(Clone, Copy, Default)]
pub struct Field {
    pub bit_offset: u32,
    pub bit_size: u32,
    pub logical_min: i32,
    pub logical_max: i32,
}

impl Field {
    pub fn present(&self) -> bool {
        self.bit_size != 0
    }

    /// The bit after the field's last; None when that does not fit 32 bits.
    pub fn end(&self) -> Option<u32> {
        self.bit_offset.checked_add(self.bit_size)
    }
}
