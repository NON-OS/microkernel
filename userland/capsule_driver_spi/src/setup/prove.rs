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
//! An LPSS function proved an SPI host before anything is written to it,
//! then readied: its LPSS capabilities register must name SPI (a read
//! alone), then the function's resets are released, chip select is taken
//! into software control and deasserted, read back to prove it holds, and
//! the core is left disabled until a transfer.

use crate::core::Regs32;
use crate::facts::*;

pub fn prove(r: &impl Regs32) -> Result<(), &'static str> {
    let caps = r.read32(PRIV_CAPS);
    if caps == 0 || caps == u32::MAX {
        return Err("spi: no LPSS capabilities register");
    }
    if caps >> CAPS_TYPE_SHIFT & CAPS_TYPE_MASK != CAPS_TYPE_SPI {
        return Err("spi: an LPSS function, not SPI");
    }
    Ok(())
}

pub fn ready(r: &impl Regs32) -> Result<(), &'static str> {
    r.write32(PRIV_RESETS, 0);
    r.write32(PRIV_RESETS, PRIV_RESETS_DEASSERT);
    r.write32(SSCR0, 0);
    r.write32(SSCR1, 0);
    chip_select(r, false)
}

/// Chip select driven by software: asserted (low) or not, read back.
pub fn chip_select(r: &impl Regs32, asserted: bool) -> Result<(), &'static str> {
    let want = CS_SW_MODE | if asserted { 0 } else { CS_HIGH };
    r.write32(PRIV_CS_CONTROL, want);
    if r.read32(PRIV_CS_CONTROL) & (CS_SW_MODE | CS_HIGH) != want {
        return Err("spi: chip select does not hold");
    }
    Ok(())
}
