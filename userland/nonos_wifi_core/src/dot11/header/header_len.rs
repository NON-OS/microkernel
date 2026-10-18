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

//! The MAC header length in bytes for a frame-control value.

use super::*;

/// The MAC header length in bytes for a frame-control value. The base header is
/// 24 bytes; a four-address frame (both ToDS and FromDS set) adds the 6-byte
/// Address4, and a QoS data frame (a data subtype with the QoS bit set) adds the
/// 2-byte QoS Control field. Real APs send QoS data, so the receive path sizes
/// the header from the frame rather than assuming the 24-byte minimum. The
/// optional HT Control field is not accounted for; the CCMP data path does not
/// set the Order bit that would carry it.
pub fn header_len(fc: u16) -> usize {
    let mut len = MAC_HEADER_LEN;
    if (fc & FC_TO_DS != 0) && (fc & FC_FROM_DS != 0) {
        len += 6;
    }
    if fc_type(fc) == TYPE_DATA && (fc_subtype(fc) & 0x08) != 0 {
        len += 2;
    }
    len
}

/// Build the 2-byte Frame Control value: protocol version 0, the given type and
/// subtype, no flags. Stored little-endian on the wire.
pub const fn frame_control(ftype: u8, subtype: u8) -> u16 {
    ((ftype as u16 & 0x3) << 2) | ((subtype as u16 & 0xF) << 4)
}

/// The type carried by a frame-control value.
pub const fn fc_type(fc: u16) -> u8 {
    ((fc >> 2) & 0x3) as u8
}

/// The subtype carried by a frame-control value.
pub const fn fc_subtype(fc: u16) -> u8 {
    ((fc >> 4) & 0xF) as u8
}

/// The sequence-control value: a 12-bit sequence number in the high bits and a
/// 4-bit fragment number of zero. Stored little-endian.
pub const fn seq_control(seq: u16) -> u16 {
    (seq & 0x0FFF) << 4
}
