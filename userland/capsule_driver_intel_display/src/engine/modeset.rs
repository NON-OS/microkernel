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

//! Lighting an output at a mode (SKL PRM Vol 12, DisplayPort and HDMI
//! enable sequences): the port first, trained for DisplayPort or clocked
//! for HDMI, then the transcoder's timing, M/N for DisplayPort, its DDI
//! function and its enable. The plane is the caller's, after this.

use super::access::{Clock, Until};
use super::error::Error;
use super::pipe::{Pipe, Transcoder};
use super::port::Port;
use super::trans_func::{enable, func_ctl, Protocol};
use super::{mn, power, regs, timing_write};
use crate::dp::caps::Caps;
use crate::dp::train::{train, Trained};
use nonos_edid::Detailed;

const STATE_US: u64 = 100_000;

/// DisplayPort or eDP on `port`, through transcoder `t` to pipe `p`.
pub fn enable_dp(
    port: &mut Port,
    t: Transcoder,
    p: Pipe,
    timing: &Detailed,
    caps: &Caps,
    max: (u32, u32),
) -> Result<Trained, Error> {
    let clock: &dyn Clock = port.clock;
    for w in power::needed(t, port.index()).into_iter().flatten() {
        power::enable(port.mmio, w, &Until::new(clock, STATE_US))?;
    }
    let link = train(port, caps, max).map_err(|_| Error::Timeout)?;
    let staged = mn::compute(timing.pixel_khz, 24, link.lanes, link.mbps).and_then(|mn| {
        timing_write::write(port.mmio, t, p, timing)?;
        mn::write(port.mmio, t, mn);
        let ctl = func_ctl(t, p, port.index(), Protocol::Dp { lanes: link.lanes }, timing);
        port.mmio.write(regs::trans_ddi_func_ctl(t), ctl);
        enable(port.mmio, t, &Until::new(clock, STATE_US))
    });
    if let Err(e) = staged {
        port.mmio.write(regs::trans_ddi_func_ctl(t), 0);
        let _ = port.take_down();
        return Err(e);
    }
    Ok(link)
}
