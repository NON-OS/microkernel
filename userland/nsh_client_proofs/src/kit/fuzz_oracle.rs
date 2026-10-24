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

//! Which host checks a fuzzed `HOST_PROOF` passes, judged from the fields
//! that built it, apart from anything the client computes.

use super::enrol::SigMode;

/// Which checks the fuzzed fields pass, judged from the fields alone.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Oracle {
    pub parses: bool,
    pub host_sig: bool,
    pub attester: bool,
    pub enrol: bool,
    pub pcrs: bool,
}

impl Oracle {
    pub fn all(&self) -> bool {
        self.parses && self.host_sig && self.attester && self.enrol && self.pcrs
    }
}

pub(super) fn mode(bits: u8) -> SigMode {
    [SigMode::Good, SigMode::BadEd, SigMode::BadMl, SigMode::Good][usize::from(bits & 3)]
}
