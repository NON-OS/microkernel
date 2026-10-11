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

pub const OP_HEALTHCHECK: u16 = 1;
pub const OP_WRITE_PCM: u16 = 7;
pub const OP_STREAM_START: u16 = 8;
pub const OP_STREAM_STOP: u16 = 9;
pub const OP_OUTPUT_STATUS: u16 = 10;

pub const E_OK: i32 = 0;
/// The queue is full: the same write is sent again a period later.
pub const E_AGAIN: i32 = -11;
pub const E_INVAL: i32 = -22;
