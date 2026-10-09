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

//! A command's response out of the bytes the chip wrote back, as
//! sd_send_cmd_get_rsp reads them: the check entry's byte first, then the
//! five SD_CMDx bytes (or sixteen ping-pong bytes for R2), then SD_STAT1.

use crate::regs::sd::{SD_CRC7_ERR, SD_NO_CHECK_CRC7, SD_RSP_TYPE_R0, SD_RSP_TYPE_R2};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Response {
    /// As Linux's cmd->resp: word 0 holds the highest bits.
    pub words: [u32; 4],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResponseError {
    /// Fewer bytes came back than the response needs.
    Short,
    /// The start and transmission bits were not both zero.
    StartBits,
    Crc7,
}

fn be32(b: &[u8]) -> u32 {
    u32::from_be_bytes([b[0], b[1], b[2], b[3]])
}

pub fn parse(results: &[u8], rsp_type: u8) -> Result<Response, ResponseError> {
    let mut out = Response { words: [0; 4] };
    if rsp_type == SD_RSP_TYPE_R0 {
        return Ok(out);
    }
    let r2 = rsp_type == SD_RSP_TYPE_R2;
    let stat_idx = if r2 { 16 } else { 5 };
    let ptr = results.get(1..1 + stat_idx + 1).ok_or(ResponseError::Short)?;
    if ptr[0] & 0xC0 != 0 {
        return Err(ResponseError::StartBits);
    }
    if rsp_type & SD_NO_CHECK_CRC7 == 0 && ptr[stat_idx] & SD_CRC7_ERR != 0 {
        return Err(ResponseError::Crc7);
    }
    if r2 {
        // The chip keeps the last byte (CRC7 and end bit); Linux puts a
        // dummy 1 there, which lands in the low bit of word 3.
        let mut b = [0u8; 17];
        b[..16].copy_from_slice(&ptr[..16]);
        b[16] = 1;
        for (i, word) in out.words.iter_mut().enumerate() {
            *word = be32(&b[1 + i * 4..]);
        }
    } else {
        out.words[0] = be32(&ptr[1..]);
    }
    Ok(out)
}
