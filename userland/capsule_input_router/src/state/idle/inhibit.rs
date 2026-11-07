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

//! Clients holding the screen awake, by pid.

use super::Idle;

impl Idle {
    /// `pid` holds the screen awake (`on`) or lets it go; false when the
    /// table is full.
    pub fn inhibit(&mut self, pid: u32, on: bool) -> bool {
        if pid == 0 {
            return false;
        }
        let at = self.inhibitors.iter().position(|&p| p == pid);
        match (on, at) {
            (true, Some(_)) => true,
            (true, None) => match self.inhibitors.iter().position(|&p| p == 0) {
                Some(free) => {
                    self.inhibitors[free] = pid;
                    true
                }
                None => false,
            },
            (false, Some(i)) => {
                self.inhibitors[i] = 0;
                true
            }
            (false, None) => true,
        }
    }

    /// A process gone lets go of the screen.
    pub fn forget(&mut self, pid: u32) {
        let _ = self.inhibit(pid, false);
    }

    /// Every holder `alive` says is gone lets go.
    pub fn purge(&mut self, alive: impl Fn(u32) -> bool) {
        for p in self.inhibitors.iter_mut().filter(|p| **p != 0) {
            if !alive(*p) {
                *p = 0;
            }
        }
    }

    pub fn inhibited(&self) -> bool {
        self.inhibitors.iter().any(|&p| p != 0)
    }
}
