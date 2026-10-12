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
//! Behind a trait so the wire's brightness op binds to it once its bytes
//! are fixed (F 12), and so a panel driven over DPCD can stand beside it.
//! Other generations' backlight registers are not cited here: ENOSYS.

use nonos_gfx_wire::errno::{EINVAL, ENOSYS};
use nonos_gfx_wire::Errno;

use super::frames::Frames;
use super::intel::Intel;
use crate::engine::access::{Clock, Mmio};
use crate::engine::backlight;
use crate::engine::gen::GEN9;
use crate::engine::pipe::Transcoder;

pub trait Brightness {
    /// The level now and the most it can be, on `output`.
    fn brightness(&mut self, output: u16) -> Result<(u32, u32), Errno>;
    /// Set `output`'s level, at most the most `brightness` gave.
    fn set_brightness(&mut self, output: u16, level: u32) -> Result<(), Errno>;
}

impl<M: Mmio, F: Frames, C: Clock> Brightness for Intel<M, F, C> {
    fn brightness(&mut self, output: u16) -> Result<(u32, u32), Errno> {
        let max = self.pwm_period(output)?;
        Ok((backlight::get(&mut self.m, max), max))
    }

    fn set_brightness(&mut self, output: u16, level: u32) -> Result<(), Errno> {
        let max = self.pwm_period(output)?;
        backlight::set(&mut self.m, level, max).map_err(|_| EINVAL)
    }
}

impl<M: Mmio, F: Frames, C: Clock> Intel<M, F, C> {
    /// The PWM's period when `output` is a Gen9 eDP panel whose PWM
    /// firmware set up; an output that is not one has no backlight here.
    fn pwm_period(&mut self, output: u16) -> Result<u32, Errno> {
        let edp = self.head(output)?.out.transcoder == Transcoder::Edp;
        if !edp || *self.gen != GEN9 {
            return Err(ENOSYS);
        }
        match backlight::period(&mut self.m) {
            0 => Err(ENOSYS),
            p => Ok(p),
        }
    }
}
