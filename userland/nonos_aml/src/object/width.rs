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

//! Integer width.

/// Integer width: 64 bits, or 32 for a definition block whose revision is
/// below 2 (section 5.2.11.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Width(pub bool);

impl Width {
    pub const WIDE: Width = Width(true);

    pub fn bytes(self) -> usize {
        if self.0 {
            8
        } else {
            4
        }
    }

    pub fn mask(self, v: u64) -> u64 {
        if self.0 {
            v
        } else {
            v & 0xFFFF_FFFF
        }
    }

    /// Ones, and the value a logical operator returns for true.
    pub fn ones(self) -> u64 {
        self.mask(u64::MAX)
    }
}
