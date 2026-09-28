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
 * Syscall numbers for the file, lock, xattr, id and usage calls, transcribed
 * from the x86_64 table.
 */

/* Files, their data and their locks; from syscall_64.tbl. */
pub const SENDFILE: u64 = 40;
pub const FLOCK: u64 = 73;
pub const FDATASYNC: u64 = 75;
pub const TRUNCATE: u64 = 76;
pub const CREAT: u64 = 85;
pub const SYNC: u64 = 162;
pub const SETXATTR: u64 = 188;
pub const LSETXATTR: u64 = 189;
pub const FSETXATTR: u64 = 190;
pub const GETXATTR: u64 = 191;
pub const LGETXATTR: u64 = 192;
pub const FGETXATTR: u64 = 193;
pub const LISTXATTR: u64 = 194;
pub const LLISTXATTR: u64 = 195;
pub const FLISTXATTR: u64 = 196;
pub const REMOVEXATTR: u64 = 197;
pub const LREMOVEXATTR: u64 = 198;
pub const FREMOVEXATTR: u64 = 199;
pub const FADVISE64: u64 = 221;
pub const FALLOCATE: u64 = 285;
pub const PREADV: u64 = 295;
pub const PWRITEV: u64 = 296;
pub const SYNCFS: u64 = 306;
pub const COPY_FILE_RANGE: u64 = 326;
pub const PREADV2: u64 = 327;
pub const PWRITEV2: u64 = 328;
pub const CLOSE_RANGE: u64 = 436;
pub const OPENAT2: u64 = 437;
pub const TIMES: u64 = 100;
