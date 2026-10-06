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

//! Output only this process's launcher reads, never the serial log.

use crate::syscall::{call_raw, N_MK_PRIVATE_WRITE};

/// Write up to 256 bytes of `buf` to this process's own output inbox and
/// nowhere else. Returns the bytes taken, -16 (EBUSY) when the inbox is
/// full and nothing was taken, or -19 (ENODEV) when there is no inbox.
pub fn mk_private_write(buf: &[u8]) -> i64 {
    call_raw(N_MK_PRIVATE_WRITE, [buf.as_ptr() as u64, buf.len() as u64, 0, 0, 0, 0])
}
