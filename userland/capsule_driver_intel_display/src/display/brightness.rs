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

//! The panel's brightness (unit 13, Intel's native half): the eDP
//! backlight's PWM in Gen9's south display (`engine::backlight`, SKL PRM
//! Vol 2c BLC_PWM_PCH_CTL1/2), a level out of the period firmware set for
//! the panel, so every step the PWM has is a level and none is invented.
//! Behind a trait so the wire's brightness op binds to it (F 12). Any
//! other output is an external monitor, whose own brightness control is
//! asked over DDC/CI (VCP 0x10); one without it, or another generation's
//! panel, whose backlight registers are not cited here, is ENOSYS.

use nonos_gfx_wire::errno::{EINVAL, ENOSYS};
use nonos_gfx_wire::Errno;

use super::frames::Frames;
use super::intel::Intel;
use crate::ddc::vcp::BRIGHTNESS;
use crate::engine::access::{Clock, Mmio};
use crate::engine::backlight;

pub trait Brightness {
    /// The level now and the most it can be, on `output`.
    fn brightness(&mut self, output: u16) -> Result<(u32, u32), Errno>;
    /// Set `output`'s level, at most the most `brightness` gave.
    fn set_brightness(&mut self, output: u16, level: u32) -> Result<(), Errno>;
}

impl<M: Mmio, F: Frames, C: Clock> Brightness for Intel<M, F, C> {
    fn brightness(&mut self, output: u16) -> Result<(u32, u32), Errno> {
        match self.pwm_period(output) {
            Ok(max) => Ok((backlight::get(&mut self.m, max), max)),
            Err(ENOSYS) => self.monitor_brightness(output),
            Err(e) => Err(e),
        }
    }

    fn set_brightness(&mut self, output: u16, level: u32) -> Result<(), Errno> {
        match self.pwm_period(output) {
            Ok(max) => backlight::set(&mut self.m, level, max).map_err(|_| EINVAL),
            Err(ENOSYS) => {
                let level = u16::try_from(level).map_err(|_| EINVAL)?;
                self.vcp_write(output, BRIGHTNESS, level)
            }
            Err(e) => Err(e),
        }
    }
}
