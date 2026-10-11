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

//! The errors a transfer answers with, as Linux's i801 returns them.

pub const E_OK: i32 = 0;
/// No device acknowledged the address (DEV_ERR).
pub const E_NXIO: i32 = -6;
/// The host failed the transaction (FAILED).
pub const E_IO: i32 = -5;
/// Arbitration lost to another master (BUS_ERR): worth asking again.
pub const E_AGAIN: i32 = -11;
/// The host was busy, or its status could not be cleared.
pub const E_BUSY: i32 = -16;
pub const E_INVAL: i32 = -22;
/// A block length the device gave outside 1 to 32.
pub const E_PROTO: i32 = -71;
/// Block transfers on a host without the 32-byte buffer (before ICH4).
pub const E_NOTSUP: i32 = -95;
pub const E_TIMEDOUT: i32 = -110;
