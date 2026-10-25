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

//! An HDMI or DVI output's clock and buffer (SKL PRM Vol 12, HDMI enable
//! sequence): the DPLL in HDMI mode at five times the pixel clock, locked
//! and routed to the DDI, then the buffer on. The buffer's drive is the
//! translation entry firmware left selected on that DDI; a port firmware
//! never lit has no entry this driver trusts and is refused.

use super::access::{rmw, Clock, Mmio, Until};
use super::ddi::{buf_ctl, buf_trans, BUF_ENABLE, TRANS_MASK};
use super::dpll::{cfgcr1, cfgcr2, enable_reg, DPLL_CTRL1, DPLL_CTRL2, DPLL_STATUS};
use super::dpll_cfgcr::cfgcr;
use super::dpll_ctrl::{ctrl1_bits, ctrl2_bits};
use super::dpll_hdmi::dco;
use super::error::Error;
use super::power::{enable, Well};
use super::wait::wait_for;

const LOCK_US: u64 = 5_000;

/// Bring DDI `ddi` up for HDMI at `pixel_khz` on DPLL `dpll` (1 to 3),
/// refusing a clock past `max_tmds_khz` (the sink's, 0 for 165 MHz).
pub fn bring_up(
    m: &mut dyn Mmio,
    clock: &dyn Clock,
    ddi: u32,
    dpll: u32,
    pixel_khz: u32,
    max_tmds_khz: u32,
) -> Result<(), Error> {
    let limit = if max_tmds_khz == 0 { 165_000 } else { max_tmds_khz };
    if pixel_khz > limit || !(1..=3).contains(&dpll) {
        return Err(Error::Argument);
    }
    let old = m.read(buf_ctl(ddi));
    if m.read(buf_trans(ddi, 0)) == 0 {
        return Err(Error::Layout);
    }
    let (c1, c2) = dco(pixel_khz).and_then(|d| cfgcr(&d)).ok_or(Error::Argument)?;
    enable(m, Well::DdiIo(ddi), &Until::new(clock, LOCK_US))?;
    let (mask, bits) = ctrl1_bits(dpll, None).ok_or(Error::Argument)?;
    rmw(m, DPLL_CTRL1, mask, bits);
    m.write(cfgcr1(dpll), c1);
    m.write(cfgcr2(dpll), c2);
    rmw(m, enable_reg(dpll), 1 << 31, 1 << 31);
    let lock = 1 << (dpll * 8);
    wait_for(m, DPLL_STATUS, lock, lock, &Until::new(clock, LOCK_US))?;
    let (mask, bits) = ctrl2_bits(ddi, dpll);
    rmw(m, DPLL_CTRL2, mask, bits);
    m.write(buf_ctl(ddi), BUF_ENABLE | (old & TRANS_MASK));
    Ok(())
}
