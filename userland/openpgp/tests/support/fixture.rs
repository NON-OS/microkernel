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

//! The vectors tools/nonos-openpgp-vectors wrote, and the file they sign.

use nonos_openpgp::{keys, Key};

pub fn file(name: &str) -> Vec<u8> {
    let path = format!("{}/tests/vectors/{name}", env!("CARGO_MANIFEST_DIR"));
    std::fs::read(&path).unwrap_or_else(|e| panic!("{path}: {e}"))
}

/// The signed file, as tools/nonos-openpgp-vectors made it.
pub fn data() -> Vec<u8> {
    (0..70000u32).map(|i| ((i * 131 + 17) % 251) as u8).collect()
}

pub fn ring(name: &str) -> Vec<Key> {
    keys(&file(&format!("{name}.pub"))).unwrap_or_else(|| panic!("{name}.pub: no keys"))
}
