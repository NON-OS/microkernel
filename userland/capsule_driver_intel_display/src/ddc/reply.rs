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

//! A Get VCP Feature reply, checked before a byte of it is believed: the
//! display's address first, a length byte saying eight, the reply opcode,
//! a result code of 0, the control asked for echoed, then maximum and
//! present value (high byte first) and a checksum seeded with REPLY_SEED.
//! A length byte of 0x80 is the null message: the display has nothing
//! to say, which is taken as no such control.

use super::error::DdcError;
use super::frame::{xor, DISPLAY, GET_VCP_REPLY, REPLY_SEED};

pub const REPLY_LEN: usize = 11;

/// What a control reads: its value now and the most it takes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Reading {
    pub present: u16,
    pub max: u16,
}

pub fn parse(code: u8, r: &[u8; REPLY_LEN]) -> Result<Reading, DdcError> {
    if r[0] != DISPLAY {
        return Err(DdcError::Reply);
    }
    if r[1] == 0x80 {
        return match xor(REPLY_SEED, &r[..3]) {
            0 => Err(DdcError::Unsupported),
            _ => Err(DdcError::Checksum),
        };
    }
    if r[1] != 0x88 {
        return Err(DdcError::Length);
    }
    if xor(REPLY_SEED, r) != 0 {
        return Err(DdcError::Checksum);
    }
    if r[2] != GET_VCP_REPLY || r[4] != code {
        return Err(DdcError::Reply);
    }
    match r[3] {
        0 => Ok(Reading {
            max: u16::from_be_bytes([r[6], r[7]]),
            present: u16::from_be_bytes([r[8], r[9]]),
        }),
        1 => Err(DdcError::Unsupported),
        _ => Err(DdcError::Reply),
    }
}
