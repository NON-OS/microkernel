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

//! The base clock the Capabilities give, or the fallback.

use super::super::regs::{
    CAP_BASE_CLK_MASK_V2, CAP_BASE_CLK_MASK_V3, CAP_BASE_CLK_SHIFT, SPEC_300,
};
use super::Caps;

/// Base clock assumed when the capabilities leave the field zero ("get the
/// information via another method"). Intel's eMMC hosts on Bay Trail through
/// Gemini Lake clock the SDHCI from 200 MHz; Linux reads the field and has
/// no Intel override, so this only matters for firmware that zeroes it.
pub const FALLBACK_BASE_MHZ: u32 = 200;

impl Caps {
    /// The base clock in Hz. Version 3.00 widened the field to bits 15:8;
    /// before it, bits 13:8. Zero means the field is not given, and the
    /// fallback applies.
    pub const fn base_hz(&self) -> u32 {
        let mask =
            if self.spec() >= SPEC_300 { CAP_BASE_CLK_MASK_V3 } else { CAP_BASE_CLK_MASK_V2 };
        let mhz = (self.caps >> CAP_BASE_CLK_SHIFT) & mask;
        let mhz = if mhz == 0 { FALLBACK_BASE_MHZ } else { mhz };
        mhz * 1_000_000
    }

    pub const fn base_given(&self) -> bool {
        let mask =
            if self.spec() >= SPEC_300 { CAP_BASE_CLK_MASK_V3 } else { CAP_BASE_CLK_MASK_V2 };
        (self.caps >> CAP_BASE_CLK_SHIFT) & mask != 0
    }
}
