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

//! Each device's last event, by name, in the order first seen; and the
//! count of each state. Fixed size: a device past the table is counted in
//! `dropped` and still has its line said.

use super::ev::{Ev, State};

pub const DEVICES: usize = 128;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Tally {
    pub devices: u32,
    pub up: u32,
    pub absent: u32,
    pub waiting: u32,
    pub failed: u32,
    pub dropped: u32,
}

pub struct Ledger {
    names: [&'static str; DEVICES],
    last: [Option<State>; DEVICES],
    len: usize,
    dropped: u32,
}

impl Ledger {
    pub const fn new() -> Self {
        Ledger { names: [""; DEVICES], last: [None; DEVICES], len: 0, dropped: 0 }
    }

    pub fn note(&mut self, dev: &'static str, ev: &Ev) {
        let at = self.names[..self.len].iter().position(|n| *n == dev);
        let at = match at {
            Some(i) => i,
            None if self.len < DEVICES => {
                self.names[self.len] = dev;
                self.len += 1;
                self.len - 1
            }
            None => {
                self.dropped = self.dropped.saturating_add(1);
                return;
            }
        };
        self.last[at] = Some(ev.state());
    }

    pub fn tally(&self) -> Tally {
        let mut t = Tally { devices: self.len as u32, dropped: self.dropped, ..Tally::default() };
        for s in self.last[..self.len].iter().flatten() {
            match s {
                State::Up => t.up += 1,
                State::Absent => t.absent += 1,
                State::Waiting => t.waiting += 1,
                State::Failed => t.failed += 1,
            }
        }
        t
    }
}

impl Default for Ledger {
    fn default() -> Self {
        Self::new()
    }
}
