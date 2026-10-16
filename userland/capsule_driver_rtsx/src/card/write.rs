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

//! Writing blocks by DMA (sd_write_long_data): the write-protect switch
//! honoured first (sdmmc_get_ro), CMD24 or CMD25 sent and its status
//! checked, the data through ADMA in an auto write that sends no command,
//! then CMD12 and the programming wait (finish.rs). A write that fails
//! part way leaves the card to be identified again before it serves
//! another.

use super::command::send_command;
use super::data::data_phase;
use super::finish::finish_write;
use super::info::Card;
use super::present::write_protected;
use super::select::select_sd;
use super::transfer::{address, end_entries, run_data, MAX_BLOCKS};
use crate::error::{Result, RtsxError};
use crate::regs::sd::*;
use crate::sd::{r1_error, Command};
use crate::setup::Driver;
use crate::wire::CmdBuf;

/// The data phase computes no CRC7, checks the CRC16 status, waits for no
/// busy end and takes no response; not a UHS card, so no write CRC timeout.
const WRITE_CFG2: u8 = SD_NO_CALCULATE_CRC7 | SD_NO_CHECK_CRC7 | SD_NO_CHECK_WAIT_CRC_TO;

/// `count` blocks from the data buffer to the card from `lba`.
pub fn write_blocks(drv: &Driver, card: &Card, lba: u32, count: u16) -> Result<()> {
    if write_protected(drv)? {
        return Err(RtsxError::WriteProtected);
    }
    let count = count.clamp(1, MAX_BLOCKS);
    let multi = count > 1;
    let status = send_command(drv, Command::write(address(card, lba)?, multi))?.words[0];
    if r1_error(status) {
        return Err(RtsxError::CardError);
    }
    let mut buf = CmdBuf::new();
    select_sd(&mut buf);
    data_phase(&mut buf, count, true);
    buf.write(SD_CFG2, 0xFF, WRITE_CFG2);
    buf.write(SD_TRANSFER, 0xFF, SD_TRANSFER_START | SD_TM_AUTO_WRITE_3);
    end_entries(&mut buf);
    let done = run_data(drv, &buf, count as u32 * 512, false);
    finish_write(drv, card, done, multi)
}
