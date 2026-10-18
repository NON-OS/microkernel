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

//! Which INTIDs exist and where each kind lives. Pure, so the decode is
//! proved on the host (gic_proofs).
//!
//! SGIs, PPIs and SPIs run 0 to 1019. 1020 to 1023 are special values the CPU
//! interface returns instead of an interrupt. LPIs start at 8192, past a
//! reserved gap, and only an ITS raises them.

/// SGIs, PPIs and SPIs: everything below the special values.
pub const LINES: u32 = 1020;
/// The first LPI.
pub const LPI_BASE: u32 = 8192;
/// LPIs the kernel hands out, from `LPI_BASE` on: enough for every vector the
/// broker binds, few enough that a handler slot apiece stays small.
pub const LPI_COUNT: u32 = 1024;
/// One handler slot per line, then one per LPI.
pub const SLOTS: usize = (LINES + LPI_COUNT) as usize;

/// The INTID an acknowledge returned, `None` for the special values, which
/// activate nothing and take no end of interrupt. Anything else was
/// activated, an LPI as much as an SPI, and has to be finished, so it is
/// passed on even when nothing will handle it.
pub const fn acknowledged(intid: u32) -> Option<u32> {
    if intid >= LINES && intid <= 1023 {
        None
    } else {
        Some(intid)
    }
}

pub const fn is_spi(intid: u32) -> bool {
    intid >= 32 && intid < LINES
}

pub const fn is_lpi(intid: u32) -> bool {
    intid >= LPI_BASE && intid - LPI_BASE < LPI_COUNT
}

/// The handler slot for `intid`, `None` for one the kernel never hands out.
pub const fn slot(intid: u32) -> Option<usize> {
    if intid < LINES {
        Some(intid as usize)
    } else if is_lpi(intid) {
        Some((LINES + (intid - LPI_BASE)) as usize)
    } else {
        None
    }
}
