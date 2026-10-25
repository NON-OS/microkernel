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

use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// The signature of the verdict at `out`, written beside it.
pub fn sig_path(out: &Path) -> PathBuf {
    beside(out, ".sig")
}

fn beside(path: &Path, suffix: &str) -> PathBuf {
    let mut p = path.as_os_str().to_owned();
    p.push(suffix);
    p.into()
}

/// Into a new file beside the target, then renamed over it: a reader never
/// sees half. The new file is created, never opened: a name planted there,
/// a link to some other file among them, is refused, not written through.
pub fn write_whole(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_nanos());
    let partial = beside(path, &format!(".{}.{nanos}.partial", std::process::id()));
    let mut o = OpenOptions::new();
    o.write(true).create_new(true).mode(0o644).custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC);
    let written = o.open(&partial).and_then(|mut f| f.write_all(bytes).and_then(|()| f.sync_all()));
    written.and_then(|()| fs::rename(&partial, path)).inspect_err(|_| {
        let _ = fs::remove_file(&partial);
    })
}
