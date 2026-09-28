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

/* The bytes moved from one descriptor to the other. */

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

use super::super::super::desc;
use super::super::super::rw::{read_at, MAX_IO};
use super::offset::read_offset;

/*
 * Read up to `count` bytes of `input` at `*offset`, or at its own offset
 * when `offset` is null, hand them to `put`, and move the right offset on.
 */
pub(super) fn move_bytes(
    guest: &mut Guest,
    input: u64,
    offset: u64,
    count: u64,
    put: impl FnOnce(&mut Guest, &[u8]) -> Result<usize, i64>,
) -> u64 {
    match guest.fds.get(input as usize).filter(|f| f.is_open()).map(|f| f.kind) {
        None => return errno::fail(errno::EBADF),
        Some(Kind::File) => {}
        Some(_) => return errno::fail(errno::EINVAL),
    }
    let at = match read_offset(guest, offset) {
        Ok(Some(at)) => at,
        Ok(None) => desc::pos(&guest.fds[input as usize]),
        Err(e) => return e,
    };
    let bytes = match read_at(guest, input, at, (count as usize).min(MAX_IO)) {
        Ok(bytes) => bytes,
        Err(e) => return errno::fail(e),
    };
    if bytes.is_empty() {
        return errno::ok(0);
    }
    let n = match put(guest, &bytes) {
        Ok(n) => n,
        Err(e) => return errno::fail(e),
    };
    let end = at + n as u64;
    if offset == 0 {
        desc::set_pos(&mut guest.fds[input as usize], end);
    } else if guest.write(offset, &end.to_le_bytes()) < 8 {
        return errno::fail(errno::EFAULT);
    }
    errno::ok(n as u64)
}
