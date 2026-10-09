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

//! `mremap` when the block cannot grow where it is: moved to fresh pages.

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Region};

use super::map_free::free_span;

/// A fresh span at the mapping cursor held like the old one, with the old
/// protection, backing and provenance; the old bytes copied in; the old span
/// gone. A move that fails part way leaves nothing new behind.
pub(super) fn moved(guest: &mut Guest, old: u64, old_len: u64, new_len: u64, like: &Region) -> u64 {
    let (at, span) = match free_span(guest, new_len) {
        Ok(got) => got,
        Err(e) => return errno::fail(e),
    };
    if guest.map_like(at, span, like) < 0 {
        return errno::fail(errno::ENOMEM);
    }
    if like.backed && !guest.copy_within(old, at, old_len) {
        let _ = guest.unmap(at, span);
        return errno::fail(errno::EFAULT);
    }
    let _ = guest.unmap(old, old_len);
    guest.mmap_next = at + span;
    errno::ok(at)
}
