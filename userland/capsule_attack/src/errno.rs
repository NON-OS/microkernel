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

//! The errno names the kernel answers with, as `abi/syscalls.toml` lists them.

pub fn name(rc: i64) -> &'static [u8] {
    match rc {
        -1 => b"EPERM",
        -2 => b"ENOENT",
        -12 => b"ENOMEM",
        -13 => b"EACCES",
        -14 => b"EFAULT",
        -16 => b"EBUSY",
        -22 => b"EINVAL",
        -38 => b"ENOSYS",
        _ => b"errno",
    }
}
