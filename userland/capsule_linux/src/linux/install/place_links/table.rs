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

//! The link table, and a package's lines taken out of it.

use alloc::vec::Vec;

use super::super::manifest::drop_links;
use crate::linux::file::{key, store_read, store_write};

pub(in super::super) const TABLE: &[u8] = b"/etc/nonos-links";

pub(in super::super) const MAX_TABLE: u32 = 64 << 10;

/// Take `links`' lines out of the link table: an uninstall's, or an install
/// that could not be recorded whole. True when none is left in it.
pub(in super::super) fn unrecord(links: &[Vec<u8>]) -> bool {
    // No table is no line to take out.
    let Ok(table) = store_read(&key(TABLE), MAX_TABLE) else { return true };
    let (table, _) = drop_links(&table, links);
    store_write(&key(TABLE), &table).is_ok()
}

/// The table as it is, or empty when there is none yet; None when the store
/// did not answer for it.
pub(super) fn current() -> Option<Vec<u8>> {
    match store_read(&key(TABLE), MAX_TABLE) {
        Ok(table) => Some(table),
        /* No table yet: this package's lines start it. */
        Err("vfs stat failed") => Some(Vec::new()),
        /*
         * A table the store did not answer for is not an empty one: written
         * over, it would hold this package's lines alone and every other
         * link, BusyBox's applets among them, would be gone for the boot.
         */
        Err(_) => None,
    }
}

/// `table` written as the link table, if it is within its ceiling.
pub(super) fn replace(table: &[u8]) -> bool {
    table.len() <= MAX_TABLE as usize && store_write(&key(TABLE), table).is_ok()
}
