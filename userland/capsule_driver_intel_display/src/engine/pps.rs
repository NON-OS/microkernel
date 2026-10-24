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

//! eDP panel power sequencing (SKL PRM Vol 2c, PP_STATUS, PP_CONTROL; the
//! delays in PP_ON_DELAYS, PP_OFF_DELAYS and PP_DIVISOR are left as
//! firmware set them for the panel). The sequencer runs the panel's own
//! power-up and power-down timing; the driver asks and waits. Bit
//! positions are the PRM's names; the write key is unverified on Gen9.

use super::access::{rmw, Expiry, Mmio};
use super::error::Error;
use super::wait::wait_for;

pub const PP_STATUS: u32 = 0xC_7200;
pub const PP_CONTROL: u32 = 0xC_7204;

/// PP_STATUS: Panel Power On Status, and Power Sequence Progress (29:28,
/// 00 idle) with Power Cycle Delay Active (27).
pub const ON: u32 = 1 << 31;
pub const BUSY: u32 = (3 << 28) | (1 << 27);

/// PP_CONTROL: Power State Target, Backlight Enable, VDD Override.
pub const TARGET_ON: u32 = 1 << 0;
pub const BACKLIGHT: u32 = 1 << 2;
pub const VDD_OVERRIDE: u32 = 1 << 3;
/// Write Protect Key, 31:16: writes take effect only with it.
pub const KEY: u32 = 0xABCD << 16;

/// Power the panel on and wait for the sequence to finish lit.
pub fn on(m: &mut dyn Mmio, until: &dyn Expiry) -> Result<(), Error> {
    if m.read(PP_STATUS) & ON != 0 {
        return Ok(());
    }
    wait_for(m, PP_STATUS, BUSY, 0, until)?;
    rmw(m, PP_CONTROL, 0xFFFF_0000 | TARGET_ON, KEY | TARGET_ON);
    wait_for(m, PP_STATUS, ON | BUSY, ON, until)
}

/// Backlight off, panel off, and wait for the sequence to finish dark.
pub fn off(m: &mut dyn Mmio, until: &dyn Expiry) -> Result<(), Error> {
    rmw(m, PP_CONTROL, 0xFFFF_0000 | TARGET_ON | BACKLIGHT | VDD_OVERRIDE, KEY);
    wait_for(m, PP_STATUS, ON | BUSY, 0, until)
}

/// The sequencer's backlight gate, beside the PWM's own enable.
pub fn backlight(m: &mut dyn Mmio, on: bool) {
    rmw(m, PP_CONTROL, 0xFFFF_0000 | BACKLIGHT, KEY | if on { BACKLIGHT } else { 0 });
}
