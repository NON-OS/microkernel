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

//! A taken-over output lit again at another mode (SKL PRM Vol 12, the
//! disable sequence, then the DisplayPort or HDMI enable sequence): the
//! output darkened, its port brought up at the new timing through the same
//! transcoder and pipe. The plane's buffer and watermarks (`buffer`) and
//! the plane itself are the caller's, after.

use super::super::access::{Clock, Mmio};
use super::super::aux_regs::Ddi;
use super::super::error::Error;
use super::super::modeset::enable_dp;
use super::super::modeset_off::{darken, enable_hdmi};
use super::super::pipe::{Pipe, Transcoder};
use super::super::port::Port;
use super::super::route::Link;
use crate::dp::caps::Caps;
use nonos_edid::Detailed;

/// Gen9's DDIs carry DisplayPort at up to HBR2 on four lanes (SKL PRM
/// Vol 12; unverified per DDI): the trainer falls back from there.
const DP_MAX: (u32, u32) = (5400, 4);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Lit {
    pub t: Transcoder,
    pub p: Pipe,
    pub ddi: u32,
    pub dpll: u32,
    pub link: Link,
}

/// Darken `lit`, then light it at `timing`; HDMI up to `tmds_khz`.
pub fn relight(
    m: &mut dyn Mmio,
    clock: &dyn Clock,
    lit: &Lit,
    timing: &Detailed,
    tmds_khz: u32,
) -> Result<(), Error> {
    let out = (lit.t, lit.p, lit.ddi, lit.dpll);
    darken(m, clock, out)?;
    match lit.link {
        Link::Dp { .. } => {
            let ddi = [Ddi::A, Ddi::B, Ddi::C, Ddi::D].get(lit.ddi as usize).copied();
            let ddi = ddi.ok_or(Error::Argument)?;
            let mut port = Port { mmio: m, ddi, dpll: lit.dpll, enhanced: false, clock };
            let caps = Caps::read(&mut port).ok().flatten().ok_or(Error::Timeout)?;
            port.enhanced = caps.enhanced_framing;
            enable_dp(&mut port, lit.t, lit.p, timing, &caps, DP_MAX).map(|_| ())
        }
        Link::Hdmi => enable_hdmi(m, clock, out, timing, tmds_khz),
        Link::Dvi => Err(Error::Argument),
    }
}
