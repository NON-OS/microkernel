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

//! An Intel function the device table does not list is taken by its class
//! (discover/classify.rs), and Intel gives that class (signal processing or
//! serial bus, subclass other) to the processor and PCH thermal subsystems
//! and the telemetry function on most laptops as well. The LPSS reset and
//! remap write at 0x204, 0x240 and 0x244, which on such a function are its
//! own registers. So an unlisted function has to prove itself an I2C core by
//! reads alone first: the DesignWare signature in IC_COMP_TYPE, and an LPSS
//! capabilities register naming I2C. Linux intel_lpss_probe likewise reads
//! the capabilities and refuses another type before intel_lpss_init_dev
//! writes anything, and binds listed ids only. A real LPSS I2C the firmware
//! left in reset reads zero here and is refused too: unlisted, it is left
//! alone rather than risk a write into a device that is not one.

use crate::constants::{
    IC_COMP_TYPE, IC_COMP_TYPE_VALUE, LPSS_DEV_I2C, LPSS_PRIV_CAPS, LPSS_PRIV_CAPS_TYPE_MASK,
    LPSS_PRIV_CAPS_TYPE_SHIFT,
};
use crate::regs::Regs;

/// Ok when the window reads as an LPSS I2C function; nothing is written.
pub fn proves_lpss_i2c(regs: Regs) -> Result<(), &'static str> {
    if regs.read32(IC_COMP_TYPE) != IC_COMP_TYPE_VALUE {
        return Err("no DesignWare signature (or held in reset)");
    }
    let caps = regs.read32(LPSS_PRIV_CAPS);
    if caps == u32::MAX
        || (caps >> LPSS_PRIV_CAPS_TYPE_SHIFT) & LPSS_PRIV_CAPS_TYPE_MASK != LPSS_DEV_I2C
    {
        return Err("LPSS capabilities do not name I2C");
    }
    Ok(())
}
