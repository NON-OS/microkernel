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

/* Where a v2 call reads or writes, and where the file ends. */

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::plain::seekable;

/*
 * Where a v2 call reads or writes: at `at`, or at the descriptor's own
 * offset (u64::MAX) for -1. With RWF_APPEND a write goes at the end, and
 * at -1 the descriptor's offset moves there first and on past what is
 * written, as Linux moves it.
 */
pub(super) fn place(guest: &mut Guest, fd: u64, at: u64, append: bool) -> Result<u64, u64> {
    match (append, at as i64) {
        (true, n) => {
            seekable(guest, fd)?;
            let end = end_of(guest, fd);
            if n != -1 {
                return Ok(end);
            }
            super::super::desc::set_pos(&mut guest.fds[fd as usize], end);
            Ok(u64::MAX)
        }
        (false, -1) => Ok(u64::MAX),
        (false, n) if n < 0 => Err(errno::fail(errno::EINVAL)),
        (false, _) => Ok(at),
    }
}

/* Where the file ends now, the family's copy's end if it holds one. */
fn end_of(guest: &Guest, fd: u64) -> u64 {
    let f = &guest.fds[fd as usize];
    super::super::cache::size(&f.path).unwrap_or(f.size)
}
