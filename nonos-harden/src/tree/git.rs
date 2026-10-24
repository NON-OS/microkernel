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

//! Asking git about the tree without letting the tree's configuration run
//! anything: no filesystem monitor, no optional index writes, paths printed
//! raw and separated by NUL so no file name can be read as two.

use std::path::Path;
use std::process::{Command, Stdio};

pub fn git(tree: &Path, args: &[&str]) -> Result<String, String> {
    let mut c = Command::new("git");
    c.arg("-C").arg(tree).args(["--no-optional-locks", "-c", "core.fsmonitor=false"]);
    let out = c.args(["-c", "core.quotePath=false"]).args(args).stdin(Stdio::null()).output();
    let out = out.map_err(|e| format!("git did not run ({e})"))?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).trim().to_string());
    }
    String::from_utf8(out.stdout).map_err(|_| "git printed something that is not UTF-8".into())
}

/// The paths a `-z` listing names.
pub fn paths(listing: &str) -> impl Iterator<Item = &str> {
    listing.split('\0').filter(|p| !p.is_empty())
}
