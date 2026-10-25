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

//! The marker that proves a reset happened (L78). A function that answers
//! again after a reset may only have been slow: a Command bit set before the
//! reset and read clear after it shows the reset reached its registers.

use super::bits_rule::{INTX_DISABLE, MEM};

/// A bit set before a reset and read clear after it: proof the function was
/// reset, not only that it answers again.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Marker {
    IntxDisable,
    MemorySpace,
}

/// INTx Disable where it reads back set, else Memory Space where it does
/// (asked only of a function with a memory BAR), else none to trust.
pub(crate) const fn marker(intx_back: bool, mem_back: bool) -> Option<Marker> {
    if intx_back {
        Some(Marker::IntxDisable)
    } else if mem_back {
        Some(Marker::MemorySpace)
    } else {
        None
    }
}

pub(crate) const fn marker_bit(m: Marker) -> u16 {
    match m {
        Marker::IntxDisable => INTX_DISABLE,
        Marker::MemorySpace => MEM,
    }
}
