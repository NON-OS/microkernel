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
//! `msync`: write a shared file mapping back to its file.
//!
//! Every file mapping here is private, since MAP_SHARED of a file is refused,
//! and a private mapping has nothing to write back: Linux answers msync on
//! one with its argument checks alone, and so does this.

use crate::linux::abi::errno;
use crate::linux::guest::{page_up, Guest, PAGE};

const MS_ASYNC: u64 = 1;
const MS_INVALIDATE: u64 = 2;
const MS_SYNC: u64 = 4;

pub fn msync(guest: &Guest, addr: u64, len: u64, flags: u64) -> u64 {
    if flags & !(MS_ASYNC | MS_INVALIDATE | MS_SYNC) != 0 || addr % PAGE != 0 {
        return errno::fail(errno::EINVAL);
    }
    if flags & MS_ASYNC != 0 && flags & MS_SYNC != 0 {
        return errno::fail(errno::EINVAL);
    }
    let Some(end) = addr.checked_add(page_up(len)).filter(|_| len <= u64::MAX - PAGE) else {
        return errno::fail(errno::ENOMEM);
    };
    // Linux reports a span with a page nothing maps as ENOMEM.
    if end > addr && guest.mapped_from(addr) < end - addr {
        return errno::fail(errno::ENOMEM);
    }
    errno::ok(0)
}
