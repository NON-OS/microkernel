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

//! Asking git about the tree without letting configuration run anything:
//! no system or global configuration or attributes, no filesystem monitor,
//! no optional index writes, and none of the environment that could point
//! git at another repository. Only plumbing that reads objects is asked
//! (rev-parse, cat-file, merge-base, ls-tree), none of which runs a filter
//! or a hook; the files themselves are hashed by this program (`blobs`).

use std::path::Path;
use std::process::{Command, Stdio};

const UNSET: [&str; 6] = [
    "GIT_DIR",
    "GIT_WORK_TREE",
    "GIT_INDEX_FILE",
    "GIT_OBJECT_DIRECTORY",
    "GIT_ALTERNATE_OBJECT_DIRECTORIES",
    "GIT_CONFIG_PARAMETERS",
];

pub fn git(tree: &Path, args: &[&str]) -> Result<String, String> {
    let mut c = Command::new("git");
    for v in UNSET {
        c.env_remove(v);
    }
    c.env("GIT_CONFIG_NOSYSTEM", "1").env("GIT_CONFIG_GLOBAL", "/dev/null");
    c.env("GIT_ATTR_NOSYSTEM", "1").env("GIT_TERMINAL_PROMPT", "0");
    c.arg("-C").arg(tree).args(["--no-optional-locks", "-c", "core.fsmonitor=false"]);
    let out = c.args(["-c", "core.quotePath=false"]).args(args).stdin(Stdio::null()).output();
    let out = out.map_err(|e| format!("git did not run ({e})"))?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).trim().to_string());
    }
    String::from_utf8(out.stdout).map_err(|_| "git printed something that is not UTF-8".into())
}
