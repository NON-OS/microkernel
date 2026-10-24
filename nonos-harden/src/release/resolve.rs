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

//! A path the record names, followed inside the release one component at a
//! time, never through a symbolic link.

use std::fs;
use std::path::{Path, PathBuf};

/// `root/rel`, when every component exists and none is a link; the last is a
/// directory if `dir`, else a regular file.
pub fn resolve(root: &Path, rel: &str, dir: bool) -> Result<PathBuf, String> {
    let mut at = root.to_path_buf();
    let parts: Vec<&str> = rel.split('/').collect();
    for (i, part) in parts.iter().enumerate() {
        at.push(part);
        let t = fs::symlink_metadata(&at).map_err(|_| format!("{rel} is missing"))?.file_type();
        let last = i + 1 == parts.len();
        if t.is_symlink() {
            return Err(format!("{rel} passes through a symbolic link"));
        }
        let want_dir = !last || dir;
        if want_dir && !t.is_dir() || !want_dir && !t.is_file() {
            let what = if want_dir { "a directory" } else { "a regular file" };
            return Err(format!("{rel} is not {what}"));
        }
    }
    Ok(at)
}
