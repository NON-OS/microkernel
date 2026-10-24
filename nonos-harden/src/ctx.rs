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

//! What the checks share: the inputs, and what the earlier checks found (the
//! record once it parsed, the kind of image it describes), so a later check
//! that depends on one says so instead of guessing.

use crate::release::record::Record;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Kind {
    /// The record did not establish it; never GO.
    Unknown,
    Release,
    Development,
}

pub struct Ctx {
    pub release: PathBuf,
    pub tree: Option<PathBuf>,
    pub record: Option<Record>,
    pub record_sha256: Option<String>,
    pub kind: Kind,
}

impl Ctx {
    pub fn new(release: PathBuf, tree: Option<PathBuf>) -> Self {
        Self { release, tree, record: None, record_sha256: None, kind: Kind::Unknown }
    }
}
