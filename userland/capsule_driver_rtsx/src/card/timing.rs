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

//! Default-speed timing (sd_set_timing's legacy case): SD 2.0 mode, the
//! fixed CRC clock with the variable sample clock, sampling on the rising
//! edge, no push-point delay.

use crate::error::Result;
use crate::regs::card::{CARD_CLK_SOURCE, CRC_FIX_CLK, SAMPLE_VAR_CLK1, SD30_VAR_CLK0};
use crate::regs::clk::{CLK_CTL, CLK_LOW_FREQ};
use crate::regs::sd::*;
use crate::setup::Driver;
use crate::wire::CmdBuf;

pub fn default_timing(drv: &Driver) -> Result<()> {
    let mut buf = CmdBuf::new();
    buf.write(SD_CFG1, SD_MODE_MASK, SD_20_MODE);
    buf.write(CLK_CTL, CLK_LOW_FREQ, CLK_LOW_FREQ);
    buf.write(CARD_CLK_SOURCE, 0xFF, CRC_FIX_CLK | SD30_VAR_CLK0 | SAMPLE_VAR_CLK1);
    buf.write(CLK_CTL, CLK_LOW_FREQ, 0);
    buf.write(SD_PUSH_POINT_CTL, 0xFF, 0);
    buf.write(SD_SAMPLE_POINT_CTL, SD20_RX_SEL_MASK, SD20_RX_POS_EDGE);
    crate::engine::send(drv, &buf, 100)
}
