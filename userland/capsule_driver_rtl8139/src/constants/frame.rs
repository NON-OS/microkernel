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

pub const MAC_LEN: usize = 6;
/// A bare header is the shortest frame taken. ARP (42 bytes) and a bare TCP
/// ACK (54) are shorter than the wire minimum, and refusing them stranded
/// IPv4 right after DHCP.
pub const MIN_ETHERNET_FRAME: usize = 14;
/// The part does not pad short frames itself, so `send` does, to this.
pub const MIN_WIRE_FRAME: usize = 60;
pub const MAX_ETHERNET_FRAME: usize = 1514;
