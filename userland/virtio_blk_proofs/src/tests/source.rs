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

//! The driver's shipping text, for the orders only a boot runs.

use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../capsule_driver_virtio_blk/src")
}

/// One file, by its path under the driver's src.
pub fn source(rel: &str) -> String {
    std::fs::read_to_string(root().join(rel)).unwrap_or_else(|_| panic!("{rel} not found"))
}

/// Where `needle` first appears in `text`.
pub fn at(text: &str, needle: &str) -> usize {
    text.find(needle).unwrap_or_else(|| panic!("`{needle}` not found"))
}

/// Every file of the driver's source, with its text.
pub fn files() -> Vec<(PathBuf, String)> {
    let (mut stack, mut out) = (vec![root()], Vec::new());
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).expect("driver source") {
            let path = entry.expect("entry").path();
            if path.is_dir() {
                stack.push(path);
            } else {
                let text = std::fs::read_to_string(&path).expect("read");
                out.push((path, text));
            }
        }
    }
    out
}
