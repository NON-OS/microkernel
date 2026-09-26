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

//! One record of the distribution's package index.

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Clone, Default)]
pub struct Pkg {
    pub name: String,
    pub version: String,
    /// SHA-1 of the package's control member, from its `C:` line.
    pub checksum: Option<[u8; 20]>,
    /// What it needs installed with it, from its `D:` line: package names
    /// and `so:` libraries, version constraints dropped.
    pub depends: Vec<String>,
}

impl Pkg {
    /// Read a `D:` line. A `!` entry is a conflict, not a need; a path is a
    /// file some other dependency installs; `cmd:` and `pc:` needs are met by
    /// whatever provides the libraries.
    pub fn read_depends(&mut self, line: &str) {
        let cut = |t: &str| String::from(t.split(['<', '>', '=', '~']).next().unwrap_or(t));
        self.depends = line
            .split_whitespace()
            .filter(|t| {
                !t.starts_with(['!', '/']) && !t.starts_with("cmd:") && !t.starts_with("pc:")
            })
            .map(cut)
            .collect();
    }
}

/// A provided name without the version it is provided at.
pub(super) fn bare(token: &str) -> String {
    String::from(token.split('=').next().unwrap_or(token))
}
