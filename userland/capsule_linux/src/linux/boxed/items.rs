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

//! The items of an `allow:` box, split into its folders and its destinations.

use alloc::vec::Vec;

use super::folders::folder;

pub fn allowed(items: &[u8]) -> Result<(Vec<Vec<u8>>, Vec<u8>), &'static str> {
    let (mut folders, mut nets) = (Vec::new(), Vec::new());
    for item in items.split(|b| *b == b',') {
        match item.strip_prefix(b"net:") {
            Some(dest) => nets.push(dest),
            None => folders.push(folder(item)?),
        }
    }
    folders.sort();
    folders.dedup();
    Ok((folders, super::dests::list(&nets)?))
}
