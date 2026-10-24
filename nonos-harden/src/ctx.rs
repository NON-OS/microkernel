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
//! record and receipt once they parsed, the kind of image, the tree's HEAD),
//! so a later check that depends on one says so instead of guessing.

use crate::release::receipt::Receipt;
use crate::release::record::Record;
use crate::tree::runner::Runner;
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
    pub runner: Box<dyn Runner>,
    pub record: Option<Record>,
    pub record_sha256: Option<String>,
    pub receipt: Option<Receipt>,
    pub tree_head: Option<String>,
    pub kind: Kind,
}

impl Ctx {
    #[cfg(test)]
    pub fn new(release: PathBuf, tree: Option<PathBuf>) -> Self {
        Self::with_runner(release, tree, Box::new(crate::tree::runner::System))
    }
    pub fn with_runner(release: PathBuf, tree: Option<PathBuf>, runner: Box<dyn Runner>) -> Self {
        let (record, record_sha256, receipt, tree_head) = (None, None, None, None);
        let kind = Kind::Unknown;
        Self { release, tree, runner, record, record_sha256, receipt, tree_head, kind }
    }
}
