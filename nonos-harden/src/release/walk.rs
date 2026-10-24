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

//! Reading inside the release without leaving it: no symbolic link is ever
//! followed, so a link planted in the release cannot point a check elsewhere.

use std::fs;
use std::path::Path;

const MAX_DEPTH: usize = 64;

/// Every regular file under `dir`, relative and sorted, at most `limit`.
pub fn files(dir: &Path, limit: usize) -> Result<Vec<String>, String> {
    let mut found = Found { limit, files: Vec::new() };
    walk(dir, "", 0, &mut found)?;
    found.files.sort();
    Ok(found.files)
}

struct Found {
    limit: usize,
    files: Vec<String>,
}

fn walk(dir: &Path, at: &str, depth: usize, out: &mut Found) -> Result<(), String> {
    if depth > MAX_DEPTH {
        return Err(format!("{at} is nested deeper than a sealed directory goes"));
    }
    let unreadable = |e: std::io::Error| format!("{at}: unreadable ({e})");
    for e in fs::read_dir(dir).map_err(unreadable)? {
        let e = e.map_err(unreadable)?;
        let not_utf8 = |_| format!("{at}: a name that is not UTF-8");
        let name = e.file_name().into_string().map_err(not_utf8)?;
        let rel = if at.is_empty() { name } else { format!("{at}/{name}") };
        let t = e.file_type().map_err(unreadable)?;
        if t.is_dir() {
            walk(&e.path(), &rel, depth + 1, out)?;
        } else if t.is_file() && out.files.len() < out.limit {
            out.files.push(rel);
        } else if t.is_file() {
            return Err("it holds more files than the record names".into());
        } else {
            return Err(format!("{rel} is a link or a special file"));
        }
    }
    Ok(())
}
