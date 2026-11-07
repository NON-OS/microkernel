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

//! A path on the wire: components below the broker's root joined by `/`.
//!
//! The broker resolves nothing. A component that could mean anything but a
//! name in its directory is refused before the tree is looked at, so no
//! path climbs above the root or names one thing two ways. The empty path
//! is the root itself.

use alloc::vec::Vec;

use crate::errno::{Errno, EINVAL, ENAMETOOLONG};
use crate::limits::{NAME_MAX, PATH_MAX};

/// The components of `path`, each checked.
pub fn parts(path: &[u8]) -> Result<Vec<&[u8]>, Errno> {
    if path.len() > PATH_MAX {
        return Err(ENAMETOOLONG);
    }
    if path.is_empty() {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    for part in path.split(|b| *b == b'/') {
        check(part)?;
        out.push(part);
    }
    Ok(out)
}

/// One name in a directory: not empty, not `.` or `..`, no NUL, no `/`, at
/// most NAME_MAX bytes.
pub fn check(name: &[u8]) -> Result<(), Errno> {
    if name.len() > NAME_MAX {
        return Err(ENAMETOOLONG);
    }
    if name.is_empty()
        || name == b"."
        || name == b".."
        || name.iter().any(|b| *b == 0 || *b == b'/')
    {
        return Err(EINVAL);
    }
    Ok(())
}
