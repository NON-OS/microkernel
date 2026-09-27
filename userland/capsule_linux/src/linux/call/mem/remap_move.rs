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
use crate::linux::guest::{span_within, Guest, MMAP_LIMIT};

const PROT_READ: u64 = 1;

// A fresh span at the mapping cursor, the old bytes copied in, the old span gone.
pub(super) fn moved(guest: &mut Guest, old: u64, old_len: u64, new_len: u64, write: bool) -> u64 {
    let Some((at, span)) = span_within(guest.mmap_next, new_len, MMAP_LIMIT) else {
        return errno::fail(errno::ENOMEM);
    };
    let Some(bytes) = guest.read(old, old_len as usize) else {
        return errno::fail(errno::EFAULT);
    };
    // Writable while the bytes go in; the old protection after.
    if guest.map(at, span, true, false) < 0 {
        return errno::fail(errno::ENOMEM);
    }
    guest.mmap_next += span;
    if guest.write(at, &bytes) < bytes.len() as i64 {
        return errno::fail(errno::EFAULT);
    }
    if !write {
        let _ = super::prot::mprotect(guest, at, span, PROT_READ);
    }
    let _ = guest.unmap(old, old_len);
    errno::ok(at)
}
