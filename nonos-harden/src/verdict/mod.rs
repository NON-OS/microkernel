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

//! The verdict: one line, GO or NO-GO, and every check behind it. Its JSON
//! form is an interface named by `format`; a change to what any field means
//! is a new format number, never a silent change.

mod build;
mod decide;
pub mod json;
pub mod text;

pub use build::build;
pub use decide::decide;
#[cfg(test)]
pub use decide::DEV_GO;

use crate::check::Check;
use crate::ctx::Kind;
use serde::Serialize;

pub const FORMAT: &str = "nonos-harden-verdict/1";
/// Said in every verdict, so one read on its own is not taken for more.
pub const SCOPE: &str = "host checks of a sealed release and the tree that built it; \
the image is not booted and no device is exercised";

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Overall {
    Go,
    NoGo,
}

#[derive(Debug, Serialize)]
pub struct Verdict {
    pub format: &'static str,
    pub tool: String,
    pub overall: Overall,
    pub verdict: String,
    pub kind: Kind,
    /// How many limits the checks carry, each listed under its check.
    pub limits: usize,
    pub release: String,
    pub record_sha256: Option<String>,
    pub tree: Option<String>,
    pub tree_head: Option<String>,
    /// The operator's public key, when the verdict is signed with it.
    pub signer: Option<String>,
    pub scope: &'static str,
    pub checks: Vec<Check>,
}
