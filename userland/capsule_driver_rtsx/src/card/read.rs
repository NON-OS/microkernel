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

//! Reading blocks by DMA (sd_read_long_data): the command and transfer
//! size into the command buffer, the DMA engine pointed at the data buffer,
//! an auto read transfer, and CMD12 after a multi-block read. A card pulled
//! during the read ends it at once (engine/wait.rs).

use super::command::put_command;
use super::data::data_phase;
use super::finish::stop_transmission;
use super::info::Card;
use super::select::select_sd;
use super::transfer::{address, end_entries, run_data, MAX_BLOCKS};
use crate::error::Result;
use crate::regs::sd::*;
use crate::sd::Command;
use crate::setup::Driver;
use crate::wire::CmdBuf;

/// `count` blocks from `lba` into the data buffer.
pub fn read_blocks(drv: &Driver, card: &Card, lba: u32, count: u16) -> Result<()> {
    let count = count.clamp(1, MAX_BLOCKS);
    let multi = count > 1;
    let cmd = Command::read(address(card, lba)?, multi);
    let mut buf = CmdBuf::new();
    select_sd(&mut buf);
    put_command(&mut buf, cmd);
    data_phase(&mut buf, count, false);
    // Not a UHS card: the write CRC timeout check is off, as Linux has it.
    buf.write(SD_CFG2, 0xFF, SD_NO_CHECK_WAIT_CRC_TO | cmd.rsp.cfg2());
    buf.write(SD_TRANSFER, 0xFF, SD_TRANSFER_START | SD_TM_AUTO_READ_2);
    end_entries(&mut buf);
    let done = run_data(drv, &buf, count as u32 * 512, true);
    if multi {
        return stop_transmission(drv, done);
    }
    done
}
