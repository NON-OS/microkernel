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

/* preadv2 and pwritev2 with their RWF_ flags. */

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::plain::at_offset;

/*
 * The v2 forms: an offset of -1 means the descriptor's own, which then
 * moves; any RWF_ flag is one this personality does not act on.
 */
pub(super) fn vectored(
    guest: &mut Guest,
    fd: u64,
    at: u64,
    flags: u64,
    go: impl FnOnce(&mut Guest) -> u64,
) -> u64 {
    if flags != 0 {
        return errno::fail(errno::EOPNOTSUPP);
    }
    match at as i64 {
        -1 => go(guest),
        n if n < 0 => errno::fail(errno::EINVAL),
        _ => at_offset(guest, fd, at, go),
    }
}
