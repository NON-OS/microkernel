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

/* truncate: the file a path names, cut or grown. */

use super::super::super::super::{at, cache, resolve, store, walk};
use super::super::advice::resize;
use crate::linux::abi::errno;
use crate::linux::guest::Guest;

pub fn truncate(guest: &Guest, path: u64, len: u64) -> u64 {
    if (len as i64) < 0 {
        return errno::fail(errno::EINVAL);
    }
    let named = match at::resolve_at(guest, super::super::super::super::flags::AT_FDCWD, path) {
        Ok(named) => named,
        Err(e) => return errno::fail(e),
    };
    let full = walk::follow(guest, named, true);
    if super::super::super::super::synth::owns(&full) {
        return errno::fail(errno::EACCES);
    }
    match store::stat(&resolve::key(&full)) {
        _ if cache::held(&full) => {}
        Ok((_, true)) => return errno::fail(errno::EISDIR),
        Ok(_) => {}
        Err(_) => return errno::fail(errno::ENOENT),
    }
    if resolve::key(&full).writable().is_err() {
        return errno::fail(errno::EROFS);
    }
    let kept = cache::held(&full);
    resize(&full, len, kept)
}
