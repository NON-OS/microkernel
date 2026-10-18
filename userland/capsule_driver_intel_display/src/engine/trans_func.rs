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

//! A transcoder's DDI function and its enable (SKL PRM Vol 2c,
//! TRANS_DDI_FUNC_CTL, TRANS_CONF). The DDI Select encoding for
//! transcoders A to C (001b DDI B through 100b DDI E; DDI A goes only
//! through the eDP transcoder) is unverified against the volume.

use super::access::{rmw, Expiry, Mmio};
use super::error::Error;
use super::pipe::{Pipe, Transcoder};
use super::regs::{trans_conf, trans_ddi_func_ctl};
use super::wait::wait_for;
use nonos_edid::Detailed;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Protocol {
    Hdmi,
    Dvi,
    /// Single-stream DisplayPort (or eDP) on `lanes` lanes.
    Dp {
        lanes: u32,
    },
}

/// The TRANS_DDI_FUNC_CTL word: enable, DDI, mode, 8 bpc, polarities,
/// the eDP transcoder's pipe, the DisplayPort width.
pub fn func_ctl(t: Transcoder, p: Pipe, ddi: u32, proto: Protocol, timing: &Detailed) -> u32 {
    let ddi_sel = if t == Transcoder::Edp { 0 } else { ddi.min(4) };
    let (mode, width) = match proto {
        Protocol::Hdmi => (0, 0),
        Protocol::Dvi => (1, 0),
        Protocol::Dp { lanes } => (2, lanes.saturating_sub(1).min(3)),
    };
    let edp_in = if t == Transcoder::Edp { 4 + p.index() } else { 0 };
    let pol = ((timing.v_positive as u32) << 17) | ((timing.h_positive as u32) << 16);
    (1 << 31) | (ddi_sel << 28) | (mode << 24) | pol | (edp_in << 12) | (width << 1)
}

/// Enable the transcoder and wait for its state bit.
pub fn enable(m: &mut dyn Mmio, t: Transcoder, until: &dyn Expiry) -> Result<(), Error> {
    rmw(m, trans_conf(t), 1 << 31, 1 << 31);
    wait_for(m, trans_conf(t), 1 << 30, 1 << 30, until)
}

/// Disable the transcoder, wait for it to stop, then clear its function.
pub fn disable(m: &mut dyn Mmio, t: Transcoder, until: &dyn Expiry) -> Result<(), Error> {
    rmw(m, trans_conf(t), 1 << 31, 0);
    wait_for(m, trans_conf(t), 1 << 30, 0, until)?;
    m.write(trans_ddi_func_ctl(t), 0);
    Ok(())
}
