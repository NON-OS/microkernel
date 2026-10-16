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

//! The ring length a controller says it supports.

use crate::constants::{
    RINGSIZE_16, RINGSIZE_2, RINGSIZE_256, RINGSIZE_CAP_16, RINGSIZE_CAP_2, RINGSIZE_CAP_256,
};

/// The size code and entry count for a CORBSIZE or RIRBSIZE value. A
/// controller that reports no sizes at all is given 256, the size every
/// controller Linux drives accepts.
pub const fn ring_size(reg: u8) -> (u8, u16) {
    if reg & RINGSIZE_CAP_256 != 0 || reg & 0xf0 == 0 {
        (RINGSIZE_256, 256)
    } else if reg & RINGSIZE_CAP_16 != 0 {
        (RINGSIZE_16, 16)
    } else if reg & RINGSIZE_CAP_2 != 0 {
        (RINGSIZE_2, 2)
    } else {
        (RINGSIZE_256, 256)
    }
}
