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

//! The operations net.anon answers.

pub const OP_HEALTHCHECK: u16 = 1;
pub const OP_STATUS: u16 = 2;
pub const OP_SYNC_DIRECTORY: u16 = 3;
pub const OP_BUILD_CIRCUIT: u16 = 4;
pub const OP_OPEN_STREAM: u16 = 5;
pub const OP_SEND: u16 = 6;
pub const OP_RECV: u16 = 7;
pub const OP_CLOSE_STREAM: u16 = 8;

/*
 * Closing a stream and closing a circuit are separate operations with separate
 * names. In the mixnet transport one call did both, and the day a rotation
 * forgot the second half every later rebind failed for the life of the boot.
 */
pub const OP_CLOSE_CIRCUIT: u16 = 9;

pub const OP_CIRCUIT_PATH: u16 = 10;
