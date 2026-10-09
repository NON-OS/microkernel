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

//! The command buffer every chip runs in rtsx_pci_init_hw after the PHY:
//! sampling, sleep state, clocks off, delink reset, drive strength, SSC on.

use crate::error::Result;
use crate::regs::card::{CARD_CLK_EN, CARD_DRIVE_DEFAULT, CARD_DRIVE_SEL};
use crate::regs::clk::{CLK_DIV, RCCTL, SSC_8X_EN, SSC_CTL1, SSC_CTL2, SSC_SEL_4M};
use crate::regs::pm::*;
use crate::setup::Driver;
use crate::wire::CmdBuf;

pub fn common_sequence(drv: &Driver) -> Result<()> {
    let mut buf = CmdBuf::new();
    // mcu_cnt of 7 so data is sampled properly.
    buf.write(CLK_DIV, 0x07, 0x07);
    buf.write(HOST_SLEEP_STATE, 0x03, 0x00);
    buf.write(CARD_CLK_EN, 0x1E, 0);
    buf.write(CHANGE_LINK_STATE, 0x0A, 0);
    buf.write(CARD_DRIVE_SEL, 0xFF, CARD_DRIVE_DEFAULT);
    buf.write(SSC_CTL1, 0xFF, SSC_8X_EN | SSC_SEL_4M);
    buf.write(SSC_CTL2, 0xFF, 0x12);
    // cd_pwr_save off, the link-ready flag cleared, a wider PERST# glitch
    // window against false card interrupts, the RC oscillator at 400 kHz,
    // and interrupt flags cleared by writing, not by reading.
    buf.write(CHANGE_LINK_STATE, 0x16, 0x10);
    buf.write(IRQSTAT0, LINK_RDY_INT, LINK_RDY_INT);
    buf.write(PERST_GLITCH_WIDTH, 0xFF, 0x80);
    buf.write(RCCTL, 0x01, 0x00);
    buf.write(NFTS_TX_CTRL, 0x02, 0);
    crate::engine::send(drv, &buf, 100)
}
