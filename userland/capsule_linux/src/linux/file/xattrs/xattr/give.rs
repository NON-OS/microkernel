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

/* An attribute's value or the list, given back as Linux gives it. */

use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

/*
 * Copy `bytes` out: size 0 asks only how long they are; a buffer too
 * small is ERANGE.
 */
pub(super) fn give(guest: &Guest, buf: u64, size: u64, bytes: Vec<u8>) -> u64 {
    if size == 0 {
        return errno::ok(bytes.len() as u64);
    }
    if (size as usize) < bytes.len() {
        return errno::fail(errno::ERANGE);
    }
    match guest.write(buf, &bytes) {
        n if n < bytes.len() as i64 => errno::fail(errno::EFAULT),
        _ => errno::ok(bytes.len() as u64),
    }
}
