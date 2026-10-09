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
//! Putting a package's files into the store, under the Linux root.

//! A package's links, written into the table the personality follows.
//!
//! The store holds files and nothing else, so a link is a `path target` line
//! in /etc/nonos-links, the table busybox's applets already resolve through.
//! A symbolic link keeps its target as written, relative or not; a hard link
//! names another member of the archive, so it becomes an absolute one.
//! Resolution passes every result through `visible`, so no link leaves the
//! guest's tree whatever it says.

use alloc::vec::Vec;

use super::manifest::drop_links;
use crate::linux::file::{key, store_read, store_write};

pub(super) const TABLE: &[u8] = b"/etc/nonos-links";
pub(super) const MAX_TABLE: u32 = 64 << 10;

/// Add `links` (path, target) to the table, skipping paths it already has.
/// The paths this added, which the package's manifest keeps so an uninstall
/// takes out its own lines and no other's, or None when the table could not
/// be written.
pub(super) fn record(links: &[(Vec<u8>, Vec<u8>)]) -> Option<Vec<Vec<u8>>> {
    if links.is_empty() {
        return Some(Vec::new());
    }
    let mut table = match store_read(&key(TABLE), MAX_TABLE) {
        Ok(table) => table,
        /* No table yet: this package's lines start it. */
        Err("vfs stat failed") => Vec::new(),
        /*
         * A table the store did not answer for is not an empty one: written
         * over, it would hold this package's lines alone and every other
         * link, BusyBox's applets among them, would be gone for the boot.
         */
        Err(_) => return None,
    };
    let mut added: Vec<Vec<u8>> = Vec::new();
    for (path, target) in links {
        if has(&table, path) || path.contains(&b' ') || path.contains(&b'\n') {
            continue;
        }
        if target.contains(&b'\n') {
            continue;
        }
        if !table.is_empty() && table.last() != Some(&b'\n') {
            table.push(b'\n');
        }
        table.extend_from_slice(path);
        table.push(b' ');
        table.extend_from_slice(target);
        table.push(b'\n');
        added.push(path.clone());
    }
    if table.len() > MAX_TABLE as usize {
        return None;
    }
    store_write(&key(TABLE), &table).ok().map(|_| added)
}

fn has(table: &[u8], path: &[u8]) -> bool {
    table
        .split(|b| *b == b'\n')
        .any(|line| line.len() > path.len() && line.starts_with(path) && line[path.len()] == b' ')
}

/// Take `links`' lines out of the link table: an uninstall's, or an install
/// that could not be recorded whole. True when none is left in it.
pub(super) fn unrecord(links: &[Vec<u8>]) -> bool {
    // No table is no line to take out.
    let Ok(table) = store_read(&key(TABLE), MAX_TABLE) else { return true };
    let (table, _) = drop_links(&table, links);
    store_write(&key(TABLE), &table).is_ok()
}
