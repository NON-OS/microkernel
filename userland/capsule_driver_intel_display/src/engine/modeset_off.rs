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

//! Lighting an HDMI output, and darkening any output (SKL PRM Vol 12,
//! HDMI enable sequence and the disable sequence: plane, transcoder,
//! function, port, clock, in that order).

use super::access::{rmw, Clock, Mmio, Until};
use super::ddi::{buf_ctl, tp_ctl};
use super::dpll::{enable_reg, DPLL_CTRL2};
use super::error::Error;
use super::pipe::{Pipe, Transcoder};
use super::plane_regs::{plane_ctl, plane_surf};
use super::trans_func::{disable, enable, func_ctl, Protocol};
use super::{cdclk, hdmi, power, regs, timing_write};
use nonos_edid::Detailed;

const STATE_US: u64 = 100_000;

/// HDMI on DDI `ddi` from DPLL `dpll`, through transcoder `t` to pipe `p`.
pub fn enable_hdmi(
    m: &mut dyn Mmio,
    clock: &dyn Clock,
    out: (Transcoder, Pipe, u32, u32),
    timing: &Detailed,
    max_tmds_khz: u32,
) -> Result<(), Error> {
    let (t, p, ddi, dpll) = out;
    if !cdclk::fits(m, timing.pixel_khz) {
        return Err(Error::Argument);
    }
    for w in power::needed(t, ddi).into_iter().flatten() {
        power::enable(m, w, &Until::new(clock, STATE_US))?;
    }
    hdmi::bring_up(m, clock, ddi, dpll, timing.pixel_khz, max_tmds_khz)?;
    timing_write::write(m, t, p, timing)?;
    m.write(regs::trans_ddi_func_ctl(t), func_ctl(t, p, ddi, Protocol::Hdmi, timing));
    enable(m, t, &Until::new(clock, STATE_US))
}

/// Plane off (latched by its surface write), transcoder off, buffer and
/// transport off, the DDI's clock and its DPLL off (DPLL 0 stays).
pub fn darken(
    m: &mut dyn Mmio,
    clock: &dyn Clock,
    out: (Transcoder, Pipe, u32, u32),
) -> Result<(), Error> {
    let (t, p, ddi, dpll) = out;
    rmw(m, plane_ctl(p, 1), 1 << 31, 0);
    let surf = m.read(plane_surf(p, 1));
    m.write(plane_surf(p, 1), surf);
    disable(m, t, &Until::new(clock, STATE_US))?;
    m.write(tp_ctl(ddi), 0);
    m.write(buf_ctl(ddi), 0);
    rmw(m, DPLL_CTRL2, 1 << (15 + ddi), 1 << (15 + ddi));
    if dpll != 0 {
        rmw(m, enable_reg(dpll), 1 << 31, 0);
    }
    Ok(())
}
