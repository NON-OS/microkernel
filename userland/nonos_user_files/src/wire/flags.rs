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

//! The flags of OPEN, RENAME and SETATTR. OPEN's are the broker's own bits, not Linux's: a client maps
//! O_ACCMODE, O_CREAT, O_EXCL, O_TRUNC, O_APPEND and O_DIRECTORY to them.

pub const READ: u32 = 1;
pub const WRITE: u32 = 2;
pub const CREATE: u32 = 4;
pub const EXCL: u32 = 8;
pub const TRUNC: u32 = 16;
pub const APPEND: u32 = 32;
pub const DIRECTORY: u32 = 64;
/// Every bit with a meaning; any other is EINVAL.
pub const KNOWN: u32 = 127;

/// RENAME's flags, Linux's own values.
pub const RENAME_NOREPLACE: u32 = 1;
pub const RENAME_EXCHANGE: u32 = 2;

/// SETATTR's mask.
pub const SET_MODE: u32 = 1;
pub const SET_MTIME: u32 = 2;
