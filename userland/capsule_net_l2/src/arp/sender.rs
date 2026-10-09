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

use crate::ethernet::MacAddress;

/// Whether `mac` can be one host's: not zero, and not a group address (the
/// first transmitted bit, the low bit of the first byte), broadcast included.
pub fn is_station(mac: &MacAddress) -> bool {
    mac[0] & 1 == 0 && *mac != [0; 6]
}

/// Whether `ip` can be a neighbour's address, for a host whose own is `ours`:
/// not unspecified (an RFC 5227 probe's sender), loopback, multicast, the
/// reserved block or broadcast, and not our own.
pub fn is_neighbour_ip(ip: &[u8; 4], ours: &[u8; 4]) -> bool {
    ip[0] != 0 && ip[0] != 127 && ip[0] < 224 && ip != ours
}
