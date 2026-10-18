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

//! The 20 byte NVGP header (section 12): magic, version, op, flags,
//! output (bytes 10-11), request id, payload length. A reply repeats the
//! header, then a status, then its body.

use crate::bytes::{put, u16_at, u32_at};
use crate::errno::{Errno, EINVAL};

pub const MAGIC: u32 = 0x4E56_4750;
pub const VERSION: u16 = 1;
pub const HDR_LEN: usize = 20;
/// The longest request, header included.
pub const MAX_REQUEST: usize = 256;
/// The longest reply body, after the header and status.
pub const MAX_REPLY_BODY: usize = 252;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Header {
    pub op: u16,
    pub flags: u16,
    /// 0 is the first output; drivers before G 10 have only that one.
    pub output: u16,
    pub request_id: u32,
    pub payload_len: u32,
}

impl Header {
    /// Write the header into the first 20 bytes of `out`.
    pub fn encode(&self, out: &mut [u8]) -> Result<(), Errno> {
        let ok = put(out, 0, &MAGIC.to_le_bytes())
            && put(out, 4, &VERSION.to_le_bytes())
            && put(out, 6, &self.op.to_le_bytes())
            && put(out, 8, &self.flags.to_le_bytes())
            && put(out, 10, &self.output.to_le_bytes())
            && put(out, 12, &self.request_id.to_le_bytes())
            && put(out, 16, &self.payload_len.to_le_bytes());
        if ok {
            Ok(())
        } else {
            Err(EINVAL)
        }
    }

    /// Read a header; the magic and version must be NVGP v1's.
    pub fn decode(b: &[u8]) -> Result<Self, Errno> {
        if u32_at(b, 0) != Some(MAGIC) || u16_at(b, 4) != Some(VERSION) {
            return Err(EINVAL);
        }
        let f = |at| u16_at(b, at).ok_or(EINVAL);
        let request_id = u32_at(b, 12).ok_or(EINVAL)?;
        let payload_len = u32_at(b, 16).ok_or(EINVAL)?;
        Ok(Header { op: f(6)?, flags: f(8)?, output: f(10)?, request_id, payload_len })
    }
}
