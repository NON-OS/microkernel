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

//! Every pseudo-terminal the family has, by the number /dev/pts gives it,
//! and the signals they raised for the serve loop to deliver. Pure.

use alloc::vec::Vec;

use super::pair::Pair;

/// The most a family has at once.
pub const MAX: usize = 64;

/// Who a raised signal is for.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum To {
    /// A process group, in the guests' numbering, as TIOCSPGRP names it.
    Group(u32),
    /// A process by the personality's own pid: a session's leader.
    Leader(u32),
}

#[derive(Default)]
pub struct Table {
    pub pairs: Vec<Option<Pair>>,
    pub signals: Vec<(To, u8)>,
}

impl Table {
    pub const fn new() -> Table {
        Table { pairs: Vec::new(), signals: Vec::new() }
    }

    /// A new pair at the lowest free number, as devpts picks one.
    pub fn make(&mut self) -> Option<u32> {
        let n = self.pairs.iter().position(Option::is_none).unwrap_or(self.pairs.len());
        if n >= MAX {
            return None;
        }
        if n == self.pairs.len() {
            self.pairs.push(None);
        }
        self.pairs[n] = Some(Pair::new());
        Some(n as u32)
    }

    pub fn get(&mut self, n: u32) -> Option<&mut Pair> {
        self.pairs.get_mut(n as usize).and_then(Option::as_mut)
    }
}
