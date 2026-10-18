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

/* symlinkat: a link made in the family's own table. */

use super::super::super::at::resolve_at;
use super::super::super::path::name_of;
use super::super::free::free_and_writable;
use crate::linux::abi::errno;
use crate::linux::guest::Guest;

pub fn symlinkat(guest: &Guest, target: u64, dirfd: u64, path: u64) -> u64 {
    /* The target first, as Linux reads it, then the new name. */
    let to = match name_of(guest, target) {
        Ok(to) => to,
        Err(e) => return errno::fail(e),
    };
    let at = match resolve_at(guest, dirfd, path) {
        Ok(at) => at,
        Err(e) => return errno::fail(e),
    };
    if let Err(e) = free_and_writable(guest, &at) {
        return errno::fail(e);
    }
    match guest.links.add(at, to) {
        Ok(()) => errno::ok(0),
        Err(e) => errno::fail(e),
    }
}
