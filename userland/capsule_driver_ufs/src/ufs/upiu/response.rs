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

//! Reading a response UPIU at the offsets in facts::upiu.

use crate::ufs::facts::upiu::*;
use crate::ufs::slot::layout::RSP_READ;

pub struct Rsp<'a>(pub &'a [u8; RSP_READ]);

impl Rsp<'_> {
    pub fn tt(&self) -> u8 {
        self.0[AT_TYPE]
    }

    /// The header's response field; for a query, its response code.
    pub fn response(&self) -> u8 {
        self.0[AT_RESPONSE]
    }

    /// The SCSI status of a command response.
    pub fn status(&self) -> u8 {
        self.0[AT_STATUS]
    }

    /// A read flag's value: bit 0 of the value field's last byte.
    pub fn flag(&self) -> bool {
        self.0[AT_FLAG] & 1 != 0
    }

    /// The sense data a CHECK CONDITION carries, cut to what was read back.
    pub fn sense(&self) -> &[u8] {
        let n = u16::from_be_bytes([self.0[AT_SENSE_LEN], self.0[AT_SENSE_LEN + 1]]);
        &self.0[AT_SENSE..(AT_SENSE + n as usize).min(RSP_READ)]
    }
}
