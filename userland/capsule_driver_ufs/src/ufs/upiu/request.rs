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

//! The three requests this driver sends: NOP OUT, a SCSI command, and a
//! flag query. Every one carries the slot's task tag.

use super::codes::{COMMAND, NOP_OUT, QUERY_REQ};
use super::header::{header, UPIU_LEN};
use crate::ufs::utp::layout::SLOT;

const TAG: u8 = SLOT as u8;

pub fn nop_out() -> [u8; UPIU_LEN] {
    header(NOP_OUT, 0, 0, TAG, 0)
}

/// A command UPIU: the expected transfer length big-endian at 12, the CDB
/// from 16, at most 16 bytes of it.
pub fn command(lun: u8, flags: u8, expected: u32, cdb: &[u8]) -> [u8; UPIU_LEN] {
    let mut out = header(COMMAND, flags, lun, TAG, 0);
    out[12..16].copy_from_slice(&expected.to_be_bytes());
    let n = core::cmp::min(cdb.len(), 16);
    out[16..16 + n].copy_from_slice(&cdb[..n]);
    out
}

/// A flag query: opcode at 12, IDN at 13, index and selector 0.
pub fn flag_query(func: u8, opcode: u8, idn: u8) -> [u8; UPIU_LEN] {
    let mut out = header(QUERY_REQ, 0, 0, TAG, func);
    out[12] = opcode;
    out[13] = idn;
    out
}
