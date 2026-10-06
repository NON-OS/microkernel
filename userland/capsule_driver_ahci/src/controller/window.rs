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

//! Which ports the mapped ABAR window reaches. PI and CAP.NP are the
//! controller's own words, and the broker may map less of ABAR than a full
//! 32-port register file (a 4 KiB BAR, or a window it clamps below an MSI-X
//! table), so a port is touched only when its whole register block lies
//! inside the mapping: a read past it would fault the capsule.

use crate::constants::port::MAX_PORTS;
use crate::constants::regs::{PORT_BASE, PORT_STRIDE};

/// A PI-shaped mask of the ports whose registers lie wholly inside an ABAR
/// mapping of `window` bytes.
pub fn ports_in_window(window: u64) -> u32 {
    let mut mask = 0u32;
    for i in 0..MAX_PORTS as u64 {
        let end = u64::from(PORT_BASE) + (i + 1) * u64::from(PORT_STRIDE);
        if end <= window {
            mask |= 1 << i;
        }
    }
    mask
}
