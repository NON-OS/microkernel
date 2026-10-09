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

//! A path out of a guest, which is a string under the store's ceiling.

use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::guest::memory::Memory;

use super::cstr::{cstr, read_cstr};

/// The vfs length prefix is one byte.
pub const MAX_PATH: usize = 255;

pub fn read_path(mem: &impl Memory, addr: u64) -> Option<Vec<u8>> {
    read_cstr(mem, addr, MAX_PATH)
}

/// The path at `addr`, or the errno Linux gives instead: EFAULT for one it
/// cannot read, ENAMETOOLONG for one past the ceiling, where answering
/// EFAULT told a program its pointer was bad when its name was too long.
pub fn path_of(mem: &impl Memory, addr: u64) -> Result<Vec<u8>, i64> {
    cstr(mem, addr, MAX_PATH)
}

/// A name a call acts on, which Linux refuses empty with ENOENT: joined
/// to the working directory, an empty name named the directory itself, so
/// an rmdir or unlink of "" acted on it.
pub fn name_of(mem: &impl Memory, addr: u64) -> Result<Vec<u8>, i64> {
    match path_of(mem, addr)? {
        name if name.is_empty() => Err(errno::ENOENT),
        name => Ok(name),
    }
}
