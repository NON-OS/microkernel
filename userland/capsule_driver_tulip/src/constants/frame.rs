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

//! Ethernet sizing, the same envelope every NIC capsule gives net.core.

const ETH_HEADER_LEN: usize = 14;
const MTU: usize = 1500;

pub const MAC_LEN: usize = 6;
/// A bare header is the shortest frame taken: ARP (42) and a bare TCP ACK
/// (54) are shorter than the wire minimum and are padded, not refused.
pub const MIN_ETHERNET_FRAME: usize = ETH_HEADER_LEN;
pub const MAX_ETHERNET_FRAME: usize = MTU + ETH_HEADER_LEN;
/// The shortest frame 802.3 puts on the wire, before the FCS. The driver pads
/// in the buffer: the 21x4x pads short frames itself, but not every model of
/// it does.
pub const MIN_WIRE_FRAME: usize = 60;
/// The part leaves the frame check sequence on every received frame and
/// counts it in the frame length, so the four bytes are cut here, as Linux
/// tulip does.
pub const FCS_LEN: usize = 4;
