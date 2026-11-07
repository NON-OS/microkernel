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

//! Writing the fixtures into a directory. Each file is made new: a file
//! already at its name is removed first (a symbolic link there is removed
//! itself, never followed), then the file is created with `O_CREAT |
//! O_EXCL | O_NOFOLLOW`, so nothing at the name can redirect the write. A
//! secret is created readable by its owner alone.

use super::files::Fixture;
use std::fs::{self, OpenOptions};
use std::io::{self, ErrorKind, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

pub fn write(dir: &Path, fixtures: &[Fixture]) -> io::Result<()> {
    fs::create_dir_all(dir)?;
    for f in fixtures {
        let mode = if f.secret { 0o600 } else { 0o644 };
        let path = dir.join(f.name);
        match fs::remove_file(&path) {
            Err(e) if e.kind() != ErrorKind::NotFound => return Err(e),
            _ => {}
        }
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .custom_flags(libc::O_NOFOLLOW)
            .mode(mode)
            .open(&path)?;
        file.write_all(&f.bytes)?;
    }
    Ok(())
}
