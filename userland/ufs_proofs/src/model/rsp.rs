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

//! The UPIUs the device answers with.

use crate::ufs::facts::upiu::RESPONSE;

pub const RSP_LEN: usize = 64;

/// A header: transaction type, function, response, SCSI status.
pub fn upiu(tt: u8, func: u8, response: u8, status: u8) -> Vec<u8> {
    let mut r = vec![0u8; RSP_LEN];
    r[0] = tt;
    r[5] = func;
    r[6] = response;
    r[7] = status;
    r
}

/// A command RESPONSE with `status`, and `sense` behind its length at 32.
pub fn command(status: u8, sense: &[u8]) -> Vec<u8> {
    let mut r = upiu(RESPONSE, 0, 0, status);
    r[32..34].copy_from_slice(&(sense.len() as u16).to_be_bytes());
    r[34..34 + sense.len()].copy_from_slice(sense);
    r
}

/// Fixed-format sense data.
pub fn sense(key: u8, asc: u8, ascq: u8) -> Vec<u8> {
    let mut s = vec![0u8; 18];
    s[0] = 0x70;
    s[2] = key;
    s[7] = 10;
    s[12] = asc;
    s[13] = ascq;
    s
}
