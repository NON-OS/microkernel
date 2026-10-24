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

//! Writing a file whole or not at all, and where a verdict's signature goes.

use std::path::{Path, PathBuf};
use std::{fs, io};

/// The signature of the verdict at `out`, written beside it.
pub fn sig_path(out: &Path) -> PathBuf {
    beside(out, ".sig")
}

fn beside(path: &Path, suffix: &str) -> PathBuf {
    let mut p = path.as_os_str().to_owned();
    p.push(suffix);
    p.into()
}

/// Beside the target, then renamed over it: a reader never sees half.
pub fn write_whole(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let partial = beside(path, ".partial");
    fs::write(&partial, bytes)?;
    fs::rename(&partial, path).inspect_err(|_| {
        let _ = fs::remove_file(&partial);
    })
}
