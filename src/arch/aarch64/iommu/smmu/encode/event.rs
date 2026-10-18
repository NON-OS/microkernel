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

//! Event queue records: 32 bytes, read as four little-endian doublewords.

pub type Event = [u64; 4];

/// What the driver says about one event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decoded {
    pub kind: u8,
    pub sid: u32,
    /// The input address for a fault on a translated access, else zero.
    pub address: u64,
}

pub const fn decode(event: &Event) -> Decoded {
    let kind = (event[0] & 0xff) as u8;
    let address = if kind >= 0x10 { event[2] } else { 0 };
    Decoded { kind, sid: (event[0] >> 32) as u32, address }
}

/// The event's name as the architecture gives it, for the log.
pub const fn name(kind: u8) -> &'static [u8] {
    match kind {
        0x01 => b"F_UUT",
        0x02 => b"C_BAD_STREAMID",
        0x03 => b"F_STE_FETCH",
        0x04 => b"C_BAD_STE",
        0x05 => b"F_BAD_ATS_TREQ",
        0x06 => b"F_STREAM_DISABLED",
        0x07 => b"F_TRANSL_FORBIDDEN",
        0x08 => b"C_BAD_SUBSTREAMID",
        0x09 => b"F_CD_FETCH",
        0x0a => b"C_BAD_CD",
        0x0b => b"F_WALK_EABT",
        0x10 => b"F_TRANSLATION",
        0x11 => b"F_ADDR_SIZE",
        0x12 => b"F_ACCESS",
        0x13 => b"F_PERMISSION",
        0x20 => b"F_TLB_CONFLICT",
        0x21 => b"F_CFG_CONFLICT",
        0x24 => b"E_PAGE_REQUEST",
        _ => b"unknown",
    }
}
