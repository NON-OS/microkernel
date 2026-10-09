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

/* Zeros written over a range of the family's copy. */

use crate::linux::abi::errno;

use super::super::super::{cache, resolve, store};
use super::punch::punched;

/*
 * Zero what of `[at, at + len)` lies inside the family's copy. The copy is
 * held first and the range cut to it (`punch`), so the zeroes are never
 * more than the held file, whatever length the caller believed it had.
 */
pub(super) fn zero(path: &[u8], at: u64, len: u64) -> u64 {
    let exists = cache::held(path) || store::stat(&resolve::key(path)).is_ok();
    if let Err(e) = cache::hold(path, exists) {
        return errno::fail(e);
    }
    let Some((from, to)) = punched(at, len, cache::size(path).unwrap_or(0)) else {
        return errno::ok(0);
    };
    let zeros = alloc::vec![0u8; (to - from) as usize];
    match cache::write(path, from, &zeros) {
        Ok(_) => errno::ok(0),
        Err(e) => errno::fail(e),
    }
}
