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

//! The DHCP parameter request list (option 55) the client sends.
//!
//! smoltcp's default asks for subnet mask, router and DNS server, [1, 3, 6],
//! which no common operating system sends, so every DHCP server and anyone
//! on the LAN could tell a NONOS machine by it. This is the list Windows 10
//! and 11 send, in their order: the most common DHCP client on the networks
//! NONOS joins, so the request looks like the largest crowd's. NONOS still
//! sends no hostname (12), vendor class (60) or FQDN (81), which Windows
//! does, and smoltcp adds a maximum message size (57) and fixes the option
//! order, which Windows does not; the match is on option 55, the field DHCP
//! fingerprinting reads first. The options asked for and not used (15, 31,
//! 33, 43, 44, 46, 47, 119, 121, 249, 252) are ignored in the reply.
pub const PARAMETER_REQUEST_LIST: [u8; 14] = [1, 3, 6, 15, 31, 33, 43, 44, 46, 47, 119, 121, 249, 252];
