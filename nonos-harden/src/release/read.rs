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

//! Reading a file the release or the tree holds: it must be a regular file
//! (not a link), and no more than one byte past `max` is read, so a file far
//! larger than what it claims to be costs nothing to refuse.

use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

pub fn read_regular(at: &Path, max: u64) -> Result<Vec<u8>, String> {
    let md = fs::symlink_metadata(at).map_err(|_| "not found".to_string())?;
    if !md.file_type().is_file() {
        return Err("not a regular file".into());
    }
    let unreadable = |e: std::io::Error| format!("unreadable ({e})");
    let mut bytes = Vec::new();
    let f = File::open(at).map_err(unreadable)?;
    f.take(max + 1).read_to_end(&mut bytes).map_err(unreadable)?;
    if bytes.len() as u64 > max {
        return Err(format!("larger than the {max} bytes it can be"));
    }
    Ok(bytes)
}
