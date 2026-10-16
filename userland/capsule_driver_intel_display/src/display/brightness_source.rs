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

//! Where an output's brightness comes from: the PWM's period on Gen9's eDP
//! panel, or the external monitor's own control over DDC/CI.

use nonos_gfx_wire::errno::ENOSYS;
use nonos_gfx_wire::Errno;

use super::frames::Frames;
use super::intel::Intel;
use crate::ddc::vcp::BRIGHTNESS;
use crate::engine::access::{Clock, Mmio};
use crate::engine::backlight;
use crate::engine::gen::GEN9;
use crate::engine::pipe::Transcoder;

impl<M: Mmio, F: Frames, C: Clock> Intel<M, F, C> {
    /// The PWM's period when `output` is a Gen9 eDP panel whose PWM
    /// firmware set up; an output that is not one has no backlight here.
    /// A monitor's own brightness over DDC/CI (`display/ddc_of.rs`); one
    /// that gives no range has none to offer.
    pub(super) fn monitor_brightness(&mut self, output: u16) -> Result<(u32, u32), Errno> {
        let r = self.vcp_read(output, BRIGHTNESS)?;
        match r.max {
            0 => Err(ENOSYS),
            max => Ok((r.present.min(max) as u32, max as u32)),
        }
    }

    pub(super) fn pwm_period(&mut self, output: u16) -> Result<u32, Errno> {
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
