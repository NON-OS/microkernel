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

//! Every way net.anon refuses, one value per cause.

pub const E_OK: u16 = 0;
pub const E_BAD_MAGIC: u16 = 1;
pub const E_BAD_VERSION: u16 = 2;
pub const E_BAD_OP: u16 = 3;
pub const E_BAD_LEN: u16 = 4;
pub const E_NO_TCP: u16 = 5;
pub const E_NO_DIRECTORY: u16 = 6;
/// A consensus was read but nothing in it can fill one of the three positions.
/// Distinct from E_NO_DIRECTORY: a different fault with a different fix.
pub const E_NO_PATH: u16 = 7;
pub const E_NO_LINK: u16 = 8;
pub const E_TABLE_FULL: u16 = 9;
pub const E_NO_CIRCUIT: u16 = 10;
pub const E_NO_STREAM: u16 = 11;
pub const E_RX_EMPTY: u16 = 14;
pub const E_STREAM_CLOSED: u16 = 15;
pub const E_DIRECTORY_STALE: u16 = 18;
/// Send window closed. The caller waits for a SENDME, it is not a failure.
pub const E_WOULD_BLOCK: u16 = 20;
