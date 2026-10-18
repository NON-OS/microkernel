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

//! A scanner that has heard nothing yet and will start its first sweep on
//! its first step.

use super::hints::Hints;
use super::sweep::Scanner;
use crate::scan_list::ScanResults;

impl Scanner {
    pub fn new() -> Self {
        let results = ScanResults::new();
        Self {
            results,
            hints: Hints::new(),
            plan: None,
            at: None,
            hop_at_ms: 0,
            steps: 0,
            raw: 0,
            beacons: 0,
        }
    }
}

impl Default for Scanner {
    fn default() -> Self {
        Self::new()
    }
}
