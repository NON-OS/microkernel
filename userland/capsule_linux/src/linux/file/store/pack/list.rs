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

//! What the packs hold below a directory, for a listing of it.

use alloc::vec::Vec;

use super::serve::name;
use super::table::with;

/// Every visible path the packs hold below `dir`.
pub fn list(dir: &[u8]) -> Vec<Vec<u8>> {
    let n = name(dir);
    with(|packs| {
        let mut out = Vec::new();
        for p in packs.iter() {
            let range = match n.is_empty() {
                true => 0..p.index.len(),
                false => p.index.under(n),
            };
            let names = range.filter_map(|i| p.index.entry(i).ok().map(|e| e.name));
            out.extend(names.map(|m| [b"/", m].concat()));
        }
        out
    })
}
