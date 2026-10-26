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

//! A kernel directory's files, each named relative to it, nested ones too:
//! for a check that has to see every file a writer could hide in.

use std::path::Path;

/// Every file under the kernel directory `rel`, named relative to it.
pub(super) fn files_under(rel: &str) -> Vec<String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../src").join(rel);
    let (mut out, mut dirs) = (Vec::new(), vec![String::new()]);
    while let Some(dir) = dirs.pop() {
        for entry in std::fs::read_dir(root.join(&dir)).unwrap_or_else(|e| panic!("{rel}: {e}")) {
            let entry = entry.expect("an entry");
            let name = format!("{dir}{}", entry.file_name().to_string_lossy());
            match entry.file_type().expect("a type").is_dir() {
                true => dirs.push(format!("{name}/")),
                false => out.push(name),
            }
        }
    }
    out
}
