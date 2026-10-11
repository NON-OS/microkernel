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
/// lba_count u64, lba_bytes u32, max_transfer_bytes u32, flags u32.
pub const OP_GEOMETRY: u16 = 2;
/// lba u64, count u32; the reply carries count * lba_bytes bytes.
pub const OP_READ: u16 = 3;
/// lba u64, count u32, then count * lba_bytes bytes.
pub const OP_WRITE: u16 = 4;
pub const OP_FLUSH: u16 = 5;
/// No payload; the reply is the 64-byte body in `identify`.
pub const OP_IDENTIFY: u16 = 6;
