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

//! The whole command over a sealed release and its tree, the tree's verifiers
//! answered by the proof's stand-in: the exit status and the JSON verdict.

use super::fake::Fake;
use super::sealed_fixture::Sealed;
use crate::cli::CheckArgs;
use crate::plan::check_with;
use serde_json::Value;
use std::fs;
use std::path::Path;

impl Sealed {
    pub fn verdict(&self, fake: Fake) -> (u8, Value) {
        self.verdict_of(self.release.path(), fake)
    }
    /// As `verdict`, over another release directory checked against this tree.
    pub fn verdict_of(&self, release: &Path, fake: Fake) -> (u8, Value) {
        let out = tempfile::tempdir().unwrap();
        let file = out.path().join("verdict.json");
        let (release, tree) = (release.into(), Some(self.tree.path().into()));
        let args = CheckArgs { release, tree, out: Some(file.clone()), sign_key: None, json: true };
        let code = check_with(&args, Box::new(fake));
        (code, serde_json::from_slice(&fs::read(file).unwrap()).unwrap())
    }
}
