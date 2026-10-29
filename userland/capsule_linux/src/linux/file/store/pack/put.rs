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

//! A pack put in and taken out through the store (swap.rs says the order).
//! This process's own hold is let go first, so it is never the one in the
//! way, and after, so the next read loads what is there now.

use nonos_libc::mk_getpid;

use super::paths::pack_path;
use super::shelf::Held;
use super::store_shelf::Store;
use super::swap::{swap, take};
use super::swap_head::in_place;
use super::table::forget;
use crate::linux::file::family::Family;

/// `pack` in as `family`'s pack of `package`, `head` its header and index
/// (swap_head.rs: a held pack naming the same files is in place already).
pub fn put(
    family: Family,
    package: &str,
    pack: &[u8],
    head: usize,
    trailer: &[u8],
) -> Result<(), Held> {
    forget();
    let (mut store, at) = (Store(mk_getpid()), pack_path(family, package));
    let put = swap(&mut store, &at, pack, trailer);
    let put = put.or_else(|why| in_place(&mut store, &at, pack, head, why));
    forget();
    put
}

/// `family`'s pack of `package` out; another family's of the same name stays.
pub fn remove(family: Family, package: &str) -> Result<(), Held> {
    forget();
    let out = take(&mut Store(mk_getpid()), &pack_path(family, package));
    forget();
    out
}
