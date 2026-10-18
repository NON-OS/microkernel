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

//! A package's links added to the table, each path once.

use alloc::vec::Vec;

use super::table::{current, replace};

/// Add `links` (path, target) to the table, skipping paths it already has.
/// The paths this added, which the package's manifest keeps so an uninstall
/// takes out its own lines and no other's, or None when the table could not
/// be written.
pub(in super::super) fn record(links: &[(Vec<u8>, Vec<u8>)]) -> Option<Vec<Vec<u8>>> {
    if links.is_empty() {
        return Some(Vec::new());
    }
    let mut table = current()?;
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
    replace(&table).then_some(added)
}

fn has(table: &[u8], path: &[u8]) -> bool {
    table
        .split(|b| *b == b'\n')
        .any(|line| line.len() > path.len() && line.starts_with(path) && line[path.len()] == b' ')
}
