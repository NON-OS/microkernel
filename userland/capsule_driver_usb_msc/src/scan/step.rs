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

//! What one step of the scanner does, and how long the server may wait for
//! a request before the next is due: the pause before the next command to a
//! device being asked, or the next look at the ports. A ready device waits
//! for requests alone.

use super::medium::Medium;
use super::scanner::Scanner;
use super::searching::SCAN_POLL_MS;
use crate::state::State;

impl Scanner {
    /// One bounded step: the next command to the device being asked, the
    /// check of one that failed, or a look at the ports and a bind.
    pub fn step(&mut self, state: &mut State) {
        if !self.found() {
            return self.decide();
        }
        match core::mem::replace(&mut self.medium, Medium::Scanning) {
            Medium::Checking(disk) => self.check(disk.xhci, disk, state),
            kept @ (Medium::Ready(_) | Medium::Changed) => self.medium = kept,
            idle => {
                self.medium = idle;
                self.search(state);
            }
        }
    }

    /// At least 1 ms; 0, as long as it takes, when the device is ready or
    /// nothing is served again.
    pub fn wait_ms(&self) -> u64 {
        let due = match (&self.medium, &self.asking) {
            (Medium::Ready(_) | Medium::Changed, _) => return 0,
            (Medium::Checking(_), _) => 0,
            (_, Some(asking)) => asking.wait_ms(),
            _ if self.known == 0 && self.searching => SCAN_POLL_MS,
            _ => self.look.left_ms(),
        };
        due.max(1)
    }
}
