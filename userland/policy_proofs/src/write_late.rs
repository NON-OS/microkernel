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

//! A vfs for the keeper's write to run against. It serves in order and acts
//! on every request it gets, as vfs does, so a late reply's work is done
//! before the next request. A reply can be late, which the write sees as a
//! failure. A keep can be refused, which does nothing, and a write refused
//! part way, which leaves the file opened and cut short.

use std::collections::HashMap;

pub struct Model {
    pub ram: HashMap<Vec<u8>, Vec<u8>>,
    pub disk: HashMap<Vec<u8>, Vec<u8>>,
    pub seed: u64,
}

#[derive(PartialEq)]
pub(crate) enum Fate {
    InTime,
    Late,
    Refused,
}

impl Model {
    pub fn new(seed: u64) -> Model {
        Model { ram: HashMap::new(), disk: HashMap::new(), seed }
    }

    /// Done and answered in time, done and answered late, or refused.
    pub(crate) fn fate(&mut self) -> Fate {
        self.seed ^= self.seed << 13;
        self.seed ^= self.seed >> 7;
        self.seed ^= self.seed << 17;
        match self.seed % 8 {
            0 | 1 => Fate::Late,
            2 => Fate::Refused,
            _ => Fate::InTime,
        }
    }
}

pub(crate) fn answer(fate: Fate) -> Result<(), ()> {
    if fate == Fate::InTime { Ok(()) } else { Err(()) }
}

#[path = "write_late_vfs.rs"]
mod calls;
