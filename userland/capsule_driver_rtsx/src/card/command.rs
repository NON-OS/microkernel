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

//! One SD command and its response (sd_send_cmd_get_rsp): the command and
//! argument into SD_CMD0 to SD_CMD4, the response type into SD_CFG2, a
//! command-and-response transfer, a check that it ended idle, then the
//! response read back from SD_CMDx (or the ping-pong buffer for R2) and
//! SD_STAT1 for the CRC.

use super::select::select_sd;
use crate::engine::{clear_error, results, send};
use crate::error::{Result, RtsxError};
use crate::regs::card::{CARD_DATA_SOURCE, PINGPONG_BUFFER};
use crate::regs::sd::*;
use crate::sd::{Command, Rsp};
use crate::setup::Driver;
use crate::wire::{parse, CmdBuf, CmdKind, Response, ResponseError};

/// SD_CMD0 holds the start bit and index, SD_CMD1 to SD_CMD4 the argument
/// most significant byte first (rtsx_pci_write_be32).
pub fn put_command(buf: &mut CmdBuf, cmd: Command) {
    buf.write(SD_CMD0, 0xFF, SD_CMD_START | cmd.index);
    for (i, b) in cmd.arg.to_be_bytes().iter().enumerate() {
        buf.write(SD_CMD0 + 1 + i as u16, 0xFF, *b);
    }
}

pub fn send_command(drv: &Driver, cmd: Command) -> Result<Response> {
    let rsp = cmd.rsp.cfg2();
    let mut buf = CmdBuf::new();
    select_sd(&mut buf);
    put_command(&mut buf, cmd);
    buf.write(SD_CFG2, 0xFF, rsp);
    buf.write(CARD_DATA_SOURCE, 0x01, PINGPONG_BUFFER);
    buf.write(SD_TRANSFER, 0xFF, SD_TM_CMD_RSP | SD_TRANSFER_START);
    let idle = SD_TRANSFER_END | SD_STAT_IDLE;
    buf.add(CmdKind::Check, SD_TRANSFER, idle, idle);
    match cmd.rsp {
        Rsp::None => {}
        Rsp::R2 => (0..16).for_each(|i| {
            buf.add(CmdKind::Read, PPBUF_BASE2 + i, 0, 0);
        }),
        _ => (SD_CMD0..=SD_CMD4).for_each(|reg| {
            buf.add(CmdKind::Read, reg, 0, 0);
        }),
    }
    buf.add(CmdKind::Read, SD_STAT1, 0, 0);
    // A busy response may hold the line while the card works (busy_timeout).
    let timeout = if cmd.rsp == Rsp::R1b { 3000 } else { 100 };
    if let Err(e) = send(drv, &buf, timeout) {
        clear_error(drv);
        return Err(e);
    }
    parse(&results(drv), rsp).map_err(|e| match e {
        ResponseError::Short => RtsxError::ResponseShort,
        ResponseError::StartBits => RtsxError::ResponseStartBits,
        ResponseError::Crc7 => RtsxError::ResponseCrc7,
    })
}
