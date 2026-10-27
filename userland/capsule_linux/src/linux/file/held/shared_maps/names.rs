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

//! A map follows its file's name. Renamed, the file is still the map's,
//! under its new name or under a directory renamed above it. Unlinked, or
//! replaced by a rename, it is not: the map keeps its pages, and a new file
//! made at that name is never taken for its. Pure.

use alloc::vec::Vec;

use super::entry::Entry;

/// `path` once `from` is renamed `to`: the file itself, or one under a
/// directory so renamed. None for any other path.
pub fn moved(path: &[u8], from: &[u8], to: &[u8]) -> Option<Vec<u8>> {
    let rest = path.strip_prefix(from)?;
    (rest.is_empty() || rest[0] == b'/').then(|| [to, rest].concat())
}

/// A rename: every map of a file at or under `from` follows it, and any
/// other at or under `to`, which the rename replaced, is orphaned.
pub fn rename(all: &mut [Entry], from: &[u8], to: &[u8]) {
    for e in all.iter_mut().filter(|e| !e.orphan) {
        match moved(&e.path, from, to) {
            Some(path) => e.path = path,
            None if moved(&e.path, to, to).is_some() => e.orphan = true,
            None => {}
        }
    }
}

/// An unlink of `path`: its maps keep their pages, and are no file's.
pub fn unlink(all: &mut [Entry], path: &[u8]) {
    all.iter_mut().filter(|e| e.shows(path)).for_each(|e| e.orphan = true);
}
