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
//! The information units the device sends on the status pipe: READ READY
//! (06h) and WRITE READY (07h), which a USB 2 device sends before a
//! command's data moves; a Sense IU (03h) that ends a command; and a
//! Response IU (04h) that answers task management or refuses an IU.

const SENSE: u8 = 0x03;
const RESPONSE: u8 = 0x04;
const READ_READY: u8 = 0x06;
const WRITE_READY: u8 = 0x07;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StatusIu<'a> {
    ReadReady {
        tag: u16,
    },
    WriteReady {
        tag: u16,
    },
    /// SCSI status, and sense data when the device gave it.
    Sense {
        tag: u16,
        status: u8,
        sense: &'a [u8],
    },
    /// 00h task management complete, 02h invalid IU, 04h not supported,
    /// 05h failed, 08h succeeded, 09h incorrect LUN, 0Ah overlapped tag.
    Response {
        tag: u16,
        code: u8,
    },
}

/// The IU in `b`, or `None` for one too short or of no kind above.
pub fn status_iu(b: &[u8]) -> Option<StatusIu<'_>> {
    let tag = u16::from_be_bytes([*b.get(2)?, *b.get(3)?]);
    match b[0] {
        READ_READY => Some(StatusIu::ReadReady { tag }),
        WRITE_READY => Some(StatusIu::WriteReady { tag }),
        SENSE if b.len() >= 16 => {
            let len = u16::from_be_bytes([b[12], b[13]]) as usize;
            let sense = b.get(16..16 + len)?;
            Some(StatusIu::Sense { tag, status: b[6], sense })
        }
        RESPONSE if b.len() >= 8 => Some(StatusIu::Response { tag, code: b[7] }),
        _ => None,
    }
}
