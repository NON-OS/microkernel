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

//! The Linux errno values the broker answers with, as positive numbers. A
//! reply carries the negated value, as a Linux system call returns it.

pub type Errno = i32;

pub const EPERM: Errno = 1;
pub const ENOENT: Errno = 2;
pub const EIO: Errno = 5;
pub const EBADF: Errno = 9;
pub const ENOMEM: Errno = 12;
pub const EACCES: Errno = 13;
pub const EBUSY: Errno = 16;
pub const EEXIST: Errno = 17;
pub const ENODEV: Errno = 19;
pub const ENOTDIR: Errno = 20;
pub const EISDIR: Errno = 21;
pub const EINVAL: Errno = 22;
pub const ENFILE: Errno = 23;
pub const EMFILE: Errno = 24;
pub const EFBIG: Errno = 27;
pub const ENOSPC: Errno = 28;
pub const ENAMETOOLONG: Errno = 36;
pub const ENOSYS: Errno = 38;
pub const ENOTEMPTY: Errno = 39;
pub const EBADMSG: Errno = 74;
pub const EMSGSIZE: Errno = 90;
pub const EUCLEAN: Errno = 117;
pub const EDQUOT: Errno = 122;
