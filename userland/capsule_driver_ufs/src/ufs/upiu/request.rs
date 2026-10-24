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

//! The three requests this engine sends: NOP OUT, a SCSI command and a
//! flag query. Each starts with the 12-byte header (transaction type,
//! flags, LUN, task tag, command set 0, query function), and each carries
//! the slot's number as its task tag.

use crate::ufs::facts::upiu::{COMMAND, NOP_OUT, QUERY_REQ, UPIU_LEN};
use crate::ufs::slot::layout::SLOT;

const TAG: u8 = SLOT as u8;

fn headed(tt: u8, flags: u8, lun: u8, func: u8) -> [u8; UPIU_LEN] {
    let mut out = [0u8; UPIU_LEN];
    (out[0], out[1], out[2], out[3], out[5]) = (tt, flags, lun, TAG, func);
    out
}

pub fn nop_out() -> [u8; UPIU_LEN] {
    headed(NOP_OUT, 0, 0, 0)
}

/// A command: the expected transfer length big-endian at 12, the CDB from
/// 16, at most 16 bytes of it.
pub fn command(lun: u8, flags: u8, expected: u32, cdb: &[u8]) -> [u8; UPIU_LEN] {
    let mut out = headed(COMMAND, flags, lun, 0);
    out[12..16].copy_from_slice(&expected.to_be_bytes());
    let n = cdb.len().min(16);
    out[16..16 + n].copy_from_slice(&cdb[..n]);
    out
}

/// A flag query: opcode at 12, flag id at 13, index and selector 0.
pub fn flag_query(func: u8, opcode: u8, idn: u8) -> [u8; UPIU_LEN] {
    let mut out = headed(QUERY_REQ, 0, 0, func);
    (out[12], out[13]) = (opcode, idn);
    out
}
