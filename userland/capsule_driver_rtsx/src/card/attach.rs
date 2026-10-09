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

//! From a powered card to one ready for reads: CID, the card's address,
//! the CSD for its size, selection, a 512-byte block length on a byte
//! addressed card, a 4-bit bus, and the default-speed 25 MHz clock. As in
//! the MMC core, these commands fail on a transfer or CRC error, not on the
//! status bits their responses carry.

use super::app::app_command;
use super::clock::switch_clock;
use super::command::send_command;
use super::info::Card;
use crate::error::{Result, RtsxError};
use crate::hw::write_register;
use crate::regs::clk::SSC_DEPTH_500K;
use crate::regs::sd::{SD_BUS_WIDTH_4BIT, SD_BUS_WIDTH_MASK, SD_CFG1};
use crate::sd::{capacity_blocks, ocr_high_capacity, r6_rca, Command};
use crate::setup::Driver;

const DEFAULT_SPEED_HZ: u32 = 25_000_000;

pub fn attach(drv: &mut Driver, ocr: u32) -> Result<Card> {
    send_command(drv, Command::ALL_SEND_CID)?;
    let rca = r6_rca(send_command(drv, Command::SEND_RELATIVE_ADDR)?.words[0]);
    let csd = send_command(drv, Command::send_csd(rca))?.words;
    let blocks = capacity_blocks(&csd).ok_or(RtsxError::UnknownCsd)?;
    send_command(drv, Command::select_card(rca))?;
    let high_capacity = ocr_high_capacity(ocr);
    if !high_capacity {
        send_command(drv, Command::SET_BLOCKLEN_512)?;
    }
    app_command(drv, rca, Command::SET_BUS_WIDTH_4)?;
    write_register(drv.regs, SD_CFG1, SD_BUS_WIDTH_MASK, SD_BUS_WIDTH_4BIT)?;
    switch_clock(drv, DEFAULT_SPEED_HZ, SSC_DEPTH_500K, false)?;
    Ok(Card { rca, high_capacity, blocks })
}
