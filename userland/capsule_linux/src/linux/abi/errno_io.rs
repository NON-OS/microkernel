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

/*
 * The errnos the file, lock and xattr calls answer with, beyond the ones
 * errno.rs has always held.
 */

pub const ENXIO: i64 = 6;
pub const EXDEV: i64 = 18;
pub const EFBIG: i64 = 27;
pub const ENODATA: i64 = 61;
pub const EOPNOTSUPP: i64 = 95;
