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

//! Syscall numbers for links, owners, times and nodes, set one of a faithful
//! install, transcribed from the x86_64 table. nr_path.rs re-exports them.

pub const LINK: u64 = 86;
pub const SYMLINK: u64 = 88;
pub const READLINKAT: u64 = 267;
pub const SYMLINKAT: u64 = 266;
pub const LINKAT: u64 = 265;
pub const RENAMEAT: u64 = 264;
pub const RENAMEAT2: u64 = 316;
pub const CHOWN: u64 = 92;
pub const FCHOWN: u64 = 93;
pub const LCHOWN: u64 = 94;
pub const FCHOWNAT: u64 = 260;
pub const UTIME: u64 = 132;
pub const UTIMES: u64 = 235;
pub const UTIMENSAT: u64 = 280;
pub const MKNOD: u64 = 133;
pub const MKNODAT: u64 = 259;
