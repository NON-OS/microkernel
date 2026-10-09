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

//! Dependency lists and pool paths, as a Packages stanza writes them.

use alloc::string::String;
use alloc::vec::Vec;

/// `libc6 (>= 2.34), libpcap0.8 | libpcap0.8t64, python3:any` as groups of
/// bare names.
pub fn needs(v: &str) -> Vec<Vec<String>> {
    let name = |t: &str| String::from(t.trim().split([' ', '(', ':', '[']).next().unwrap_or(""));
    v.split(',')
        .map(|g| g.split('|').map(name).filter(|n| !n.is_empty()).collect::<Vec<_>>())
        .filter(|g| !g.is_empty())
        .collect()
}

/// A pool path that stays under the mirror's root.
pub fn safe(path: &str) -> bool {
    let chars = path.bytes().all(|b| b.is_ascii_alphanumeric() || b"._+~-/:%".contains(&b));
    let parts = path.split('/').all(|c| !c.is_empty() && c != "." && c != "..");
    chars && parts && path.ends_with(".deb")
}
