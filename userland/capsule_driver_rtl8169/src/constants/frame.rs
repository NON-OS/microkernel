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

const ETH_HEADER_LEN: usize = 14;
const MTU: usize = 1500;

pub const MAC_LEN: usize = 6;
/// A bare header is the shortest frame taken; ARP (42) and a bare TCP ACK (54)
/// are shorter than the wire minimum, and refusing them stranded IPv4.
pub const MIN_ETHERNET_FRAME: usize = ETH_HEADER_LEN;
/// `send` pads to this: several 8168 revisions do not pad short frames right
/// (Linux pads them in software for the same reason).
pub const MIN_WIRE_FRAME: usize = 60;
pub const MAX_ETHERNET_FRAME: usize = MTU + ETH_HEADER_LEN;
