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

//! Reading a file the release or the tree holds. It is opened once, without
//! following a link at its last component and without blocking on a FIFO, and
//! what was opened must be a regular file: every check reads the bytes it
//! judges through this handle, never by opening the path a second time. No
//! more than one byte past `max` is read, so an oversized file costs nothing.

use std::fs::{File, OpenOptions};
use std::io::Read;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

/// The open file and its length, when `at` is a regular file and not a link.
pub fn open_regular(at: &Path) -> Result<(File, u64), String> {
    let mut o = OpenOptions::new();
    o.read(true).custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK | libc::O_CLOEXEC);
    let f = o.open(at).map_err(|e| match e.raw_os_error() {
        Some(libc::ELOOP) => "a symbolic link".to_string(),
        Some(libc::ENOENT) => "not found".to_string(),
        _ => format!("unreadable ({e})"),
    })?;
    let md = f.metadata().map_err(|e| format!("unreadable ({e})"))?;
    match md.file_type().is_file() {
        true => Ok((f, md.len())),
        false => Err("not a regular file".into()),
    }
}

pub fn read_regular(at: &Path, max: u64) -> Result<Vec<u8>, String> {
    let (f, len) = open_regular(at)?;
    if len > max {
        return Err(format!("larger than the {max} bytes it can be"));
    }
    let mut bytes = Vec::new();
    f.take(max + 1).read_to_end(&mut bytes).map_err(|e| format!("unreadable ({e})"))?;
    if bytes.len() as u64 > max {
        return Err(format!("larger than the {max} bytes it can be"));
    }
    Ok(bytes)
}
