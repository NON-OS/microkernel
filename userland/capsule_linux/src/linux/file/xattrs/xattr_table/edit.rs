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

/* An attribute removed or listed, and attributes that follow their file. */

use alloc::vec::Vec;

use crate::linux::abi::errno;

use super::table::ATTRS;

pub fn remove(path: &[u8], name: &[u8]) -> Result<(), i64> {
    let mut all = ATTRS.0.borrow_mut();
    let before = all.len();
    all.retain(|(p, n, _)| !(p == path && n == name));
    if all.len() == before {
        Err(errno::ENODATA)
    } else {
        Ok(())
    }
}

/* Every name, each followed by a NUL, as listxattr gives them. */
pub fn list(path: &[u8]) -> Vec<u8> {
    let all = ATTRS.0.borrow();
    all.iter()
        .filter(|(p, _, _)| p == path)
        .flat_map(|(_, n, _)| n.iter().copied().chain([0]))
        .collect()
}

pub fn forget(path: &[u8]) {
    ATTRS.0.borrow_mut().retain(|(p, _, _)| p != path);
}

pub fn renamed(from: &[u8], to: &[u8]) {
    let mut all = ATTRS.0.borrow_mut();
    all.retain(|(p, _, _)| p != to);
    for (p, _, _) in all.iter_mut().filter(|(p, _, _)| p == from) {
        *p = to.to_vec();
    }
}
