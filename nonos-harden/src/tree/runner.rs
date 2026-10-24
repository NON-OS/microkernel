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

//! Running the tree's own verifiers. Each runs with no input, its output to a
//! log, and a time limit: a verifier that never ends is stopped and counted as
//! not having run, with every process it started, so a hung tool cannot hold
//! the verdict back or pass it.

use std::path::{Path, PathBuf};
use std::time::Duration;

pub struct Cmd {
    pub program: String,
    pub args: Vec<String>,
    pub cwd: PathBuf,
    pub env: Vec<(String, String)>,
    pub limit: Duration,
}

/// How a command ended: its exit code (none if a signal or the limit ended
/// it) and the last of what it printed.
pub struct Done {
    pub code: Option<i32>,
    pub tail: String,
}

pub trait Runner {
    fn run(&self, cmd: &Cmd, log: &Path) -> Result<Done, String>;
}

pub use super::system::System;
