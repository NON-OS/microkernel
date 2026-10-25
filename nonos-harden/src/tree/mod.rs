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

//! The checks over the tree that built the image: that it is the release's
//! source, that its trust ledger holds, and the tree's own verifiers run on it.

mod blob_same;
mod blobs;
pub mod declared;
pub mod git;
pub mod inputs;
pub mod judge;
pub mod ledger;
pub mod manifest;
pub mod matrix;
pub mod report;
pub mod runner;
pub mod scratch;
pub mod source;
pub mod summary;
mod system;
mod untracked;
pub mod verify;

use std::path::{Path, PathBuf};

/// The checkout `start` lies in: the nearest directory at or above it that
/// holds the verifiers this command runs.
pub fn find(start: &Path) -> Option<PathBuf> {
    let holds = |d: &&Path| {
        d.join("nonos-verify/Cargo.toml").is_file() && d.join("tools/nonos-verify-matrix").is_file()
    };
    start.ancestors().find(holds).map(Path::to_path_buf)
}

/// The tree to check against, as an absolute path: the one given, or else
/// the one the command is run inside. Never one at or inside the release: a
/// release must not bring the verifiers that judge it. Every verifier runs
/// from a directory of its own, so a relative path would name another place.
pub fn choose(given: Option<PathBuf>, release: &Path) -> Result<Option<PathBuf>, String> {
    let here = || std::env::current_dir().ok().and_then(|d| find(&d));
    let Some(tree) = given.or_else(here) else { return Ok(None) };
    let (t, r) = (std::fs::canonicalize(&tree), std::fs::canonicalize(release));
    match (t, r) {
        (Ok(t), Ok(r)) if t.starts_with(&r) => {
            Err(format!("the tree {} lies inside the release it would judge", tree.display()))
        }
        (Ok(t), _) => Ok(Some(t)),
        (Err(e), _) => Err(format!("the tree {} cannot be opened ({e})", tree.display())),
    }
}

/// The release directory as an absolute path, or as given when it cannot be
/// opened, so `release.record` says so.
pub fn absolute(release: &Path) -> PathBuf {
    std::fs::canonicalize(release).unwrap_or_else(|_| release.to_path_buf())
}
