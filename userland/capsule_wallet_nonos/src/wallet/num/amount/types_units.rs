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

//! A figure made from base units, as "use all" fills the field.

use super::Amount;

impl Amount {
    /// `v` base units at `dp` decimals as a figure, exactly, its trailing
    /// zeros after the point taken off: what "use all" fills the field with.
    pub fn of_units(v: u128, dp: u32) -> Self {
        let (mut value, mut places) = (v, dp);
        while places > 0 && value % 10 == 0 {
            value /= 10;
            places -= 1;
        }
        let digits = value.checked_ilog10().map_or(1, |d| d + 1);
        Amount { value, typed: digits.max(places + 1), places, point: places > 0 }
    }
}
