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

//! The eDP backlight's PWM in the south display (SKL PRM Vol 2c,
//! BLC_PWM_PCH_CTL1 and BLC_PWM_PCH_CTL2): CTL2 holds the period in 31:16
//! and the duty cycle in 15:0, both in PWM clocks; CTL1 bit 31 enables
//! it. Firmware sets the period for the panel; a level is a share of it.

use super::access::{rmw, Mmio};
use super::error::Error;

pub const BLC_PWM_PCH_CTL1: u32 = 0xC_8250;
pub const BLC_PWM_PCH_CTL2: u32 = 0xC_8254;
pub const PWM_ENABLE: u32 = 1 << 31;

/// The period firmware programmed, 0 when it left the PWM unset.
pub fn period(m: &mut dyn Mmio) -> u32 {
    m.read(BLC_PWM_PCH_CTL2) >> 16
}

/// The level now, as `level` out of `max`.
pub fn get(m: &mut dyn Mmio, max: u32) -> u32 {
    let ctl2 = m.read(BLC_PWM_PCH_CTL2);
    let p = ctl2 >> 16;
    if p == 0 {
        return 0;
    }
    ((ctl2 & 0xFFFF).min(p) as u64 * max as u64 / p as u64) as u32
}

/// Set the duty cycle to `level` of `max` of the period and enable the
/// PWM; refused when firmware left no period to share.
pub fn set(m: &mut dyn Mmio, level: u32, max: u32) -> Result<(), Error> {
    let p = period(m);
    if p == 0 || max == 0 || level > max {
        return Err(Error::Argument);
    }
    let duty = (p as u64 * level as u64 / max as u64) as u32;
    rmw(m, BLC_PWM_PCH_CTL2, 0xFFFF, duty);
    rmw(m, BLC_PWM_PCH_CTL1, PWM_ENABLE, PWM_ENABLE);
    Ok(())
}
