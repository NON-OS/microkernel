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

//! The display core clock firmware chose (SKL PRM Vol 2c, CDCLK_CTL: the
//! CD Frequency Decimal field, 10:0, holds the frequency in MHz times two
//! less one, in 0.5 MHz steps; field placement unverified). A pipe's
//! pixel rate may not pass it. This driver keeps firmware's CDCLK, whose
//! change runs through the power controller, and refuses a faster mode.

use super::access::Mmio;

pub const CDCLK_CTL: u32 = 0x4_6000;

/// CDCLK in kHz, 0 when the field reads as unset.
pub fn khz(m: &mut dyn Mmio) -> u32 {
    let dec = m.read(CDCLK_CTL) & 0x7FF;
    if dec == 0 {
        return 0;
    }
    (dec + 1) * 500
}

/// A pixel clock this CDCLK can drive one pipe at.
pub fn fits(m: &mut dyn Mmio, pixel_khz: u32) -> bool {
    let c = khz(m);
    c != 0 && pixel_khz <= c
}
