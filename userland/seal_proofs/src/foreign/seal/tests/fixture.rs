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

//! A live family as reserve() makes one, with fixed randomness.

use crate::foreign::seal::family::Family;

pub const SUP: u32 = 41;
pub const SALT: [u64; 4] = [5, 6, 7, 8];

pub fn family(sup: u32) -> Family {
    let mut f = Family::fresh(sup, ([0xAB; 32], 0x1_0400_1979, 0), [9; 32], SALT);
    f.enrolled = true;
    f
}
