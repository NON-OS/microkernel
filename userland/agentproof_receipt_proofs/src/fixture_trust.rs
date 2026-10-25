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
//! What a verifier that pinned the fixture's AK and release trusts.

use agentproof_receipt::trust::{Release, Supervisor, Trust};
use nsh_attest::Doc;

use crate::build::ak::Key;
use crate::fixture::{Fixture, CAPS};

impl Fixture {
    /// What a verifier that pinned this fixture's AK and release trusts.
    pub fn trust(&mut self) -> Trust {
        let doc = self.spec.doc_bytes();
        let d = Doc::parse(&doc).unwrap();
        let pcr = |i| *d.pcr(i).unwrap();
        let release =
            Release { version: "0.9.3-test".into(), pcr8: pcr(8), pcr9: pcr(9), pcr14: pcr(14) };
        let ak = Key::new(self.spec.doc_key).public();
        Trust {
            releases: vec![release],
            supervisors: vec![Supervisor { meas: [5; 32], caps: CAPS }],
            ak,
        }
    }
}
