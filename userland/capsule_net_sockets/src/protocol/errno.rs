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

pub const E_OK: u16 = 0;
pub const E_BAD_MAGIC: u16 = 1;
pub const E_BAD_VERSION: u16 = 2;
pub const E_BAD_OP: u16 = 3;
pub const E_BAD_LEN: u16 = 4;
pub const E_NO_HANDLE: u16 = 5;
pub const E_NO_TRANSPORT: u16 = 6;
pub const E_TABLE_FULL: u16 = 7;
pub const E_BAD_FAMILY: u16 = 8;
pub const E_BAD_KIND: u16 = 9;
pub const E_NOT_BOUND: u16 = 10;
pub const E_NOT_CONNECTED: u16 = 12;
pub const E_ALREADY_BOUND: u16 = 13;
pub const E_BAD_ADDR: u16 = 14;
/// A connect by name on a mixnet socket. Its frames carry an address, not a
/// name, and resolving the name here would send it to net.dns in the clear.
pub const E_NAME_REFUSED: u16 = 15;
/// A connect by name whose lookup net.dns could not make: it did not answer,
/// or it said no upstream server answered it. Not a name that has no
/// address (that is E_BAD_ADDR): no DNS server is reachable, so every name
/// fails the same way until the network comes back.
pub const E_NO_DNS: u16 = 16;
