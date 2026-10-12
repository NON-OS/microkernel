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

//! One timer interrupt as the GTDT gives it: a GSIV and its flags.

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Timer {
    pub gsiv: u32,
    /// Bit 0 edge-triggered, bit 1 active low, bit 2 always on.
    pub flags: u32,
}

impl Timer {
    pub fn edge(&self) -> bool {
        self.flags & 1 != 0
    }
    pub fn active_low(&self) -> bool {
        self.flags & 2 != 0
    }
}
