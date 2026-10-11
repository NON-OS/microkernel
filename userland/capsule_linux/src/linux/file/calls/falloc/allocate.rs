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

/* What a checked fallocate does to the file. */

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::super::super::cache;
use super::super::size::resize;
use super::calls::PUNCH_HOLE;
use super::zero::zero;

/// Descriptor `fd`'s file at `path`, `size` long as it last knew, given
/// `len` bytes from `at` as `mode` says.
pub(super) fn allocate(
    guest: &mut Guest,
    fd: u64,
    mode: u64,
    (at, len): (u64, u64),
    (path, size): (&[u8], u64),
) -> u64 {
    let now = cache::size(path).unwrap_or(size);
    let want = at.saturating_add(len);
    match mode {
        /* The descriptor learns the new length only once the copy has it. */
        0 if want > now => {
            let done = resize(path, want, true);
            if let Some(e) = guest.fds.get_mut(fd as usize).filter(|_| done == errno::ok(0)) {
                e.size = want;
            }
            done
        }
        m if m & PUNCH_HOLE != 0 && at < now => zero(path, at, len),
        _ => errno::ok(0),
    }
}
