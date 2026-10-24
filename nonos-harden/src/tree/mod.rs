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
