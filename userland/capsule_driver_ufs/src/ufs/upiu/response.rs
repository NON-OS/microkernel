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

//! What came back: the transaction type the device answered with, its
//! response and status, a query's response code and flag value, and a
//! command's sense data (struct utp_cmd_rsp).

use crate::ufs::utp::layout::RSP_READ;

pub struct Rsp<'a>(pub &'a [u8; RSP_READ]);

impl Rsp<'_> {
    pub fn tt(&self) -> u8 {
        self.0[0]
    }

    /// The header's response field; for a query, its response code.
    pub fn response(&self) -> u8 {
        self.0[6]
    }

    /// The SCSI status of a command response.
    pub fn status(&self) -> u8 {
        self.0[7]
    }

    /// A READ FLAG answer: bit 0 of the value field's last byte.
    pub fn flag(&self) -> bool {
        self.0[23] & 1 != 0
    }

    /// The sense data a CHECK CONDITION carries: its length big-endian at
    /// 32, the bytes from 34, cut to what was read back.
    pub fn sense(&self) -> &[u8] {
        let n = u16::from_be_bytes([self.0[32], self.0[33]]) as usize;
        &self.0[34..core::cmp::min(34 + n, RSP_READ)]
    }
}
