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

/* An attribute set, as setxattr's flags allow. */

use alloc::vec::Vec;

use crate::linux::abi::errno;

use super::table::{ATTRS, SIZE_MAX, XATTR_CREATE, XATTR_REPLACE};

pub fn set(path: &[u8], name: &[u8], value: Vec<u8>, flags: u64) -> Result<(), i64> {
    if flags & !(XATTR_CREATE | XATTR_REPLACE) != 0 {
        return Err(errno::EINVAL);
    }
    if value.len() > SIZE_MAX {
        return Err(7); /* E2BIG */
    }
    let mut all = ATTRS.0.borrow_mut();
    let at = all.iter().position(|(p, n, _)| p == path && n == name);
    match (at, flags) {
        (Some(_), XATTR_CREATE) => Err(errno::EEXIST),
        (None, XATTR_REPLACE) => Err(errno::ENODATA),
        (Some(i), _) => {
            all[i].2 = value;
            Ok(())
        }
        (None, _) => {
            all.push((path.to_vec(), name.to_vec(), value));
            Ok(())
        }
    }
}
