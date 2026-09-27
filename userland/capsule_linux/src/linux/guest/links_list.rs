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

//! Links as a directory listing sees them.

use alloc::string::String;
use alloc::vec::Vec;

use super::links::Links;

impl Links {
    /// The names of the links directly inside `dir`, so a listing shows them.
    pub fn names_in(&self, dir: &[u8]) -> Vec<String> {
        let dir = if dir == b"/" { &b""[..] } else { dir };
        let leaf = |from: &[u8]| from.strip_prefix(dir)?.strip_prefix(b"/").map(|l| l.to_vec());
        let all = self.0.borrow();
        let names = all.iter().filter_map(|(from, _)| leaf(from));
        names.filter(|l| !l.contains(&b'/')).filter_map(|l| String::from_utf8(l).ok()).collect()
    }
}
