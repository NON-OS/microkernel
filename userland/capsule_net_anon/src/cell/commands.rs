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

//! Cell and relay command numbers, read out of the fork.

// Read out of the fork's src/core/or/or.h; unchanged from upstream Tor.

pub const CELL_DESTROY: u8 = 4;
pub const CELL_RELAY: u8 = 3;
pub const CELL_RELAY_EARLY: u8 = 9;
pub const CELL_CREATE2: u8 = 10;
pub const CELL_CREATED2: u8 = 11;

pub const CELL_VERSIONS: u8 = 7;
pub const CELL_NETINFO: u8 = 8;
pub const CELL_VPADDING: u8 = 128;
pub const CELL_CERTS: u8 = 129;
pub const CELL_AUTH_CHALLENGE: u8 = 130;

pub const RELAY_BEGIN: u8 = 1;
pub const RELAY_DATA: u8 = 2;
pub const RELAY_END: u8 = 3;
pub const RELAY_CONNECTED: u8 = 4;
pub const RELAY_SENDME: u8 = 5;
pub const RELAY_TRUNCATED: u8 = 9;
pub const RELAY_EXTEND2: u8 = 14;
pub const RELAY_EXTENDED2: u8 = 15;

/// The ntor handshake's number in a CREATE2 or EXTEND2 cell.
pub const HANDSHAKE_NTOR: u16 = 0x0002;

/// True for the commands that arrive as variable length cells.
///
pub fn is_variable(command: u8) -> bool {
    command == CELL_VERSIONS || command >= CELL_VPADDING
}
