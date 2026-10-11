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

//! The 12-byte UPIU header (struct utp_upiu_header): transaction type,
//! flags, LUN, task tag, command set type (0, SCSI) and the query function
//! for a query; EHS length, device information and a data segment of 0.

/// NOP OUT, command and query request UPIUs are all 32 bytes.
pub const UPIU_LEN: usize = 32;

pub fn header(tt: u8, flags: u8, lun: u8, tag: u8, func: u8) -> [u8; UPIU_LEN] {
    let mut out = [0u8; UPIU_LEN];
    out[0] = tt;
    out[1] = flags;
    out[2] = lun;
    out[3] = tag;
    out[5] = func;
    out
}
