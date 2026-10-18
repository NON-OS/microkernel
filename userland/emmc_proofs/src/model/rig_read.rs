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

//! What a test reads back from a rig: the model's complaints, the commands
//! it saw, the driver's log and the time.

use super::host::CmdRec;
use super::rig::Rig;

impl Rig {
    pub fn violations(&self) -> Vec<String> {
        self.sim.borrow().violations.clone()
    }

    pub fn cmds(&self) -> Vec<CmdRec> {
        self.sim.borrow().cmds.clone()
    }

    pub fn indices(&self) -> Vec<u8> {
        self.cmds().iter().map(|c| c.index).collect()
    }

    pub fn logs(&self) -> Vec<String> {
        self.log.0.borrow().clone()
    }

    pub fn logged(&self, needle: &str) -> bool {
        self.logs().iter().any(|l| l.contains(needle))
    }

    pub fn ms(&self) -> u64 {
        self.now.get()
    }

    /// Fail the test with the model's complaints and the driver's log.
    pub fn assert_clean(&self) {
        let v = self.violations();
        assert!(v.is_empty(), "model violations: {v:#?}\nlog: {:#?}", self.logs());
    }
}
