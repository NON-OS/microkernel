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

//! The HCI UART transport (Vol 4, Part A): each packet is sent after a
//! one-octet packet indicator naming its kind (Part A, 2), and the octets
//! after it are cut by the length the packet's own header states.
//!
//! The transport has no frame delimiter and no checksum, so nothing marks
//! where a packet starts except the end of the last one. An octet that is
//! not a packet indicator where one must be means the two sides no longer
//! agree on packet edges. Part A, 3 leaves recovery to a reset, so that is
//! an error the host sees, never something skipped over.

/// HCI Command packet (Part A, 2).
pub const COMMAND: u8 = 0x01;
/// HCI ACL Data packet.
pub const ACL: u8 = 0x02;
/// HCI Synchronous Data packet.
pub const SYNC: u8 = 0x03;
/// HCI Event packet.
pub const EVENT: u8 = 0x04;
/// HCI ISO Data packet.
pub const ISO: u8 = 0x05;

/// The header length, where its length field sits and how wide it is, and
/// the longest packet, for each kind a controller sends (Vol 4, Part E,
/// 5.4.2 to 5.4.5). `acl_max` is the controller's own ACL bound. A command
/// never comes from a controller, so it has none.
pub fn shape(indicator: u8, acl_max: u16) -> Option<(usize, usize)> {
    match indicator {
        EVENT => Some((2, 2 + 255)),
        ACL => Some((4, 4 + usize::from(acl_max))),
        SYNC => Some((3, 3 + 255)),
        ISO => Some((4, 4 + 0x3fff)),
        _ => None,
    }
}

/// The data length a header of kind `indicator` states.
pub fn stated(indicator: u8, head: &[u8]) -> usize {
    match indicator {
        EVENT => usize::from(head[1]),
        SYNC => usize::from(head[2]),
        // The ISO data length is 14 bits; the top two are reserved.
        ISO => usize::from(u16::from_le_bytes([head[2], head[3]]) & 0x3fff),
        _ => usize::from(u16::from_le_bytes([head[2], head[3]])),
    }
}
