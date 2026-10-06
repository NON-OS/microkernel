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

/* The family's extended attributes, by path. */

use alloc::vec::Vec;
use core::cell::RefCell;

use crate::linux::abi::errno;

pub(super) const XATTR_CREATE: u64 = 1;

pub(super) const XATTR_REPLACE: u64 = 2;

/* XATTR_SIZE_MAX. */
pub(super) const SIZE_MAX: usize = 65536;

const NAMESPACES: [&[u8]; 3] = [b"user.", b"trusted.", b"security."];

pub(super) struct Attrs(pub(super) RefCell<Vec<(Vec<u8>, Vec<u8>, Vec<u8>)>>);

/*
 * SAFETY: one personality process serves one family from one serve loop,
 * answering one call at a time, so no two borrows can overlap.
 */
unsafe impl Sync for Attrs {}

pub(super) static ATTRS: Attrs = Attrs(RefCell::new(Vec::new()));

/* The name is one tmpfs keeps: a known namespace and something after it. */
pub fn check_name(name: &[u8]) -> Result<(), i64> {
    if name.is_empty() || name.len() > 255 {
        return Err(errno::ERANGE);
    }
    match NAMESPACES.iter().any(|ns| name.len() > ns.len() && name.starts_with(ns)) {
        true => Ok(()),
        false => Err(errno::EOPNOTSUPP),
    }
}

pub fn get(path: &[u8], name: &[u8]) -> Result<Vec<u8>, i64> {
    let all = ATTRS.0.borrow();
    let found = all.iter().find(|(p, n, _)| p == path && n == name);
    found.map(|(_, _, v)| v.clone()).ok_or(errno::ENODATA)
}
