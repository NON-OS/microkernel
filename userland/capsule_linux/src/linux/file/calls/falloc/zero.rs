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

/* Zero [from, to) of the family's copy. */
pub(super) fn zero(path: &[u8], from: u64, to: u64) -> u64 {
    let exists = cache::held(path) || store::stat(&resolve::key(path)).is_ok();
    let zeros = alloc::vec![0u8; (to - from) as usize];
    match cache::hold(path, exists).and_then(|()| cache::write(path, from, &zeros)) {
        Ok(_) => errno::ok(0),
        Err(e) => errno::fail(e),
    }
}
