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

/* openat. */

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::super::flags::O_CLOEXEC;
use super::super::path;
use super::mark::mark;
use super::named::open_named;

pub fn openat(guest: &mut Guest, dirfd: u64, path_ptr: u64, flags: u64, mode: u64) -> u64 {
    let name = match path::name_of(guest, path_ptr) {
        Ok(name) => name,
        Err(e) => return errno::fail(e),
    };
    let named = match super::super::at::named_at(guest, dirfd, &name) {
        Ok(named) => named,
        Err(e) => return errno::fail(e),
    };
    let got = open_named(guest, named, flags, mode);
    mark(guest, got, flags & O_CLOEXEC != 0);
    got
}
