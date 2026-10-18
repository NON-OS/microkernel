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

//! What an uninstall takes out: the paths no other package lists, and the
//! link table without its lines.

use alloc::vec::Vec;

use super::format::Manifest;

/// `mine` less every path another installed package also lists: a file two
/// packages both wrote stays until neither needs it.
pub fn only_mine(
    mine: &[Vec<u8>],
    others: &[Manifest],
    pick: fn(&Manifest) -> &[Vec<u8>],
) -> Vec<Vec<u8>> {
    mine.iter().filter(|p| !others.iter().any(|o| pick(o).contains(p))).cloned().collect()
}

/// The link table without the lines for `paths`, and how many went.
pub fn drop_links(table: &[u8], paths: &[Vec<u8>]) -> (Vec<u8>, usize) {
    let mut out = Vec::with_capacity(table.len());
    let mut dropped = 0usize;
    for line in table.split(|b| *b == b'\n').filter(|l| !l.is_empty()) {
        let path = line.split(|b| *b == b' ').next().unwrap_or(line);
        if paths.iter().any(|p| p.as_slice() == path) {
            dropped += 1;
            continue;
        }
        out.extend_from_slice(line);
        out.push(b'\n');
    }
    (out, dropped)
}
