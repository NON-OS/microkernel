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

//! The ops, by number. Fixed fields before the path, in order.

/// path → attr
pub const STAT: u16 = 1;
/// flags u32, mode u32, path → handle u64, attr
pub const OPEN: u16 = 2;
/// handle u64 →
pub const CLOSE: u16 = 3;
/// handle u64, offset u64, length u32 → bytes
pub const READ: u16 = 4;
/// handle u64, offset u64, bytes → written u32, size u64
pub const WRITE: u16 = 5;
/// handle u64 → attr
pub const FSTAT: u16 = 6;
/// handle u64, size u64 →
pub const FTRUNCATE: u16 = 7;
/// handle u64 →
pub const FSYNC: u16 = 8;
/// size u64, path →
pub const TRUNCATE: u16 = 9;
/// flags u32, from length u16, from, to →
pub const RENAME: u16 = 10;
/// path →
pub const UNLINK: u16 = 11;
/// mode u32, path → attr
pub const MKDIR: u16 = 12;
/// path →
pub const RMDIR: u16 = 13;
/// cookie u64, max u32, path → next u64, end u8, count u32,
/// then per entry ino u64, kind u8, name length u8, name
pub const LIST: u16 = 14;
/// mask u32, mode u32, mtime u64, path → attr
pub const SETATTR: u16 = 15;
/// → quota u64, used u64, nodes u64, max nodes u64, chunk u32
pub const STATFS: u16 = 16;
