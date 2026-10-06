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

//! Setting the card clock (rtsx_pci_switch_clock): SD_CFG1's divider, then,
//! when the SSC clock changes, the SSC programmed under CLK_LOW_FREQ and
//! released once it has settled (SSC_CLOCK_STABLE_WAIT, 130 us).

use crate::clock::sleep_ms;
use crate::error::{Result, RtsxError};
use crate::hw::write_register;
use crate::regs::clk::*;
use crate::regs::sd::{SD_CFG1, SD_CLK_DIVIDE_MASK};
use crate::setup::Driver;
use crate::wire::{plan, CmdBuf};

pub fn switch_clock(drv: &mut Driver, hz: u32, depth: u8, initial: bool) -> Result<()> {
    // double_clk is set for every timing but SDR50 and SDR104, which this
    // driver does not use.
    let p = plan(hz, depth, initial, true).ok_or(RtsxError::ClockRange)?;
    write_register(drv.regs, SD_CFG1, SD_CLK_DIVIDE_MASK, p.divider)?;
    if p.clk_mhz == drv.cur_clock {
        return Ok(());
    }
    let mut buf = CmdBuf::new();
    buf.write(CLK_CTL, CLK_LOW_FREQ, CLK_LOW_FREQ);
    buf.write(CLK_DIV, 0xFF, (p.div << 4) | p.mcu_cnt);
    buf.write(SSC_CTL1, SSC_RSTB, 0);
    buf.write(SSC_CTL2, SSC_DEPTH_MASK, p.ssc_depth);
    buf.write(SSC_DIV_N_0, 0xFF, p.n);
    buf.write(SSC_CTL1, SSC_RSTB, SSC_RSTB);
    crate::engine::send(drv, &buf, 2000)?;
    sleep_ms(1);
    write_register(drv.regs, CLK_CTL, CLK_LOW_FREQ, 0)?;
    drv.cur_clock = p.clk_mhz;
    Ok(())
}
