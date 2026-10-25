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
//! Registers as a map with a log of every read and write. Status bits
//! follow their request bits `lag` reads later, never when `lag` is None,
//! and never for a (register, bit) in `stuck`. PHY group writes fan out.

use std::collections::{BTreeMap, BTreeSet};

pub use super::follow::CONFS;

#[derive(Default)]
pub struct Regs {
    pub map: BTreeMap<u32, u32>,
    pub writes: Vec<(u32, u32)>,
    pub reads: Vec<u32>,
    pub lag: Option<u32>,
    pub stuck: BTreeSet<(u32, u32)>,
    pub(super) pending: BTreeMap<(u32, u32), u32>,
}

impl Regs {
    pub fn new(lag: Option<u32>) -> Self {
        Regs { lag, ..Default::default() }
    }

    pub fn get(&self, reg: u32) -> u32 {
        self.map.get(&reg).copied().unwrap_or(0)
    }

    pub fn set(&mut self, reg: u32, v: u32) {
        self.map.insert(reg, v);
    }

    pub(super) fn lag_of(&self, reg: u32, bit: u32) -> Option<u32> {
        if self.stuck.contains(&(reg, bit)) {
            None
        } else {
            self.lag
        }
    }
}
