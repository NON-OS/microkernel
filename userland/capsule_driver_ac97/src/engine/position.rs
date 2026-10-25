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

//! Where the engine is. The engine moves to the next entry when one is
//! played unless it is on the last valid one, where it halts; the driver
//! never polls more than a few milliseconds apart, and each look pulls the
//! last-valid index up to the entry just behind the current one, thirty-one
//! entries (over a second) ahead of where it plays. A halted engine is
//! started again by that same write.

use crate::constants::bus::{PO_CIV, PO_LVI};
use crate::constants::ring::{INDEX_MASK, LVI_LAG};
use crate::regs::Port;

/// The entry the engine plays.
pub fn follow(bus: &Port) -> u8 {
    let civ = bus.read8(PO_CIV) & INDEX_MASK;
    bus.write8(PO_LVI, civ.wrapping_add(LVI_LAG) & INDEX_MASK);
    civ
}
