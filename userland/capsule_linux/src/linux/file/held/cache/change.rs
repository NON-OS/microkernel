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

/* Writing into a copy, and cutting or growing it. */

use crate::linux::abi::errno;

use super::table::{now, with, MAX_FILE};

/* Write `bytes` at `at`, filling any gap with zeros, as a sparse write reads. */
pub fn write(path: &[u8], at: u64, bytes: &[u8]) -> Result<usize, i64> {
    let end = (at as usize).checked_add(bytes.len()).filter(|e| *e <= MAX_FILE);
    let end = end.ok_or(errno::EFBIG)?;
    with(path, |e| {
        if e.data.len() < end {
            e.data.resize(end, 0);
        }
        e.data[at as usize..end].copy_from_slice(bytes);
        e.dirty = true;
        e.mtime_ms = now();
        bytes.len()
    })
    .ok_or(errno::EBADF)
}

pub fn resize(path: &[u8], len: u64) -> Result<(), i64> {
    let len = usize::try_from(len).ok().filter(|l| *l <= MAX_FILE).ok_or(errno::EFBIG)?;
    with(path, |e| {
        e.data.resize(len, 0);
        e.dirty = true;
        e.mtime_ms = now();
    })
    .ok_or(errno::EBADF)
}
