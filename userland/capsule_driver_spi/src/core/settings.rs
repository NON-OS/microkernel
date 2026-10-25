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
//! A transfer's settings in the core's terms: the bit clock as a divider
//! of the input clock, never faster than asked, and the SPI mode as clock
//! polarity and phase (mode bit 1 CPOL, bit 0 CPHA).

use crate::facts::{SSCR0_DSS_MASK, SSCR0_SCR_MAX, SSCR0_SCR_SHIFT, SSCR1_SPH, SSCR1_SPO};

/// SCR for `want_hz` from an `input_hz` clock: the bit clock is input over
/// SCR + 1, rounded so it never exceeds what the device asked for; `None`
/// when even the largest divider runs it faster than that.
pub fn scr_for(input_hz: u32, want_hz: u32) -> Option<u32> {
    let scr = input_hz.div_ceil(want_hz.max(1)).saturating_sub(1);
    (scr <= SSCR0_SCR_MAX).then_some(scr)
}

/// SSCR0 for 8-bit Motorola SPI frames at `scr`, the port still disabled.
pub fn sscr0(scr: u32) -> u32 {
    ((8 - 1) & SSCR0_DSS_MASK) | scr << SSCR0_SCR_SHIFT
}

/// SSCR1 for SPI `mode` 0 to 3, interrupts left off.
pub fn sscr1(mode: u8) -> u32 {
    let mut v = 0;
    if mode & 2 != 0 {
        v |= SSCR1_SPO;
    }
    if mode & 1 != 0 {
        v |= SSCR1_SPH;
    }
    v
}
