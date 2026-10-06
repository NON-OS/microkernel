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
//! Memory calls the Linux x86_64 table has that the first families lacked.

pub const MSYNC: u64 = 26;
pub const MINCORE: u64 = 27;
pub const MLOCK: u64 = 149;
pub const MUNLOCK: u64 = 150;
pub const MLOCKALL: u64 = 151;
pub const MUNLOCKALL: u64 = 152;
pub const MLOCK2: u64 = 325;
