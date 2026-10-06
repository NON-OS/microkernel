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


//! The port map: which of the three integer ports is busy in which cycle.
//! A uop is placed in the first cycle with a free port it can use, trying
//! P5, then P0, then P1, so that ports any instruction can use do not crowd
//! out P1, the only one that multiplies.

use super::template::{Template, PORT_NONE, PORT_P0, PORT_P1, PORT_P5};

/// Instructions are generated until this cycle.
pub const TARGET_CYCLE: i32 = 192;
const PORT_MAP_SIZE: i32 = TARGET_CYCLE + 4;

pub struct Ports {
    busy: [[u8; 3]; PORT_MAP_SIZE as usize],
}

impl Ports {
    pub fn new() -> Self {
        Self { busy: [[0; 3]; PORT_MAP_SIZE as usize] }
    }

    /// The cycle a uop can start in at or after `cycle`, reserving the port
    /// when `commit` is set; `None` when the map runs out.
    fn uop(&mut self, uop: u8, start: i32, commit: bool) -> Option<i32> {
        for cycle in start.max(0)..PORT_MAP_SIZE {
            let slot = &mut self.busy[cycle as usize];
            for (port, flag) in [(2usize, PORT_P5), (0, PORT_P0), (1, PORT_P1)] {
                if uop & flag != 0 && slot[port] == 0 {
                    if commit {
                        slot[port] = uop;
                    }
                    return Some(cycle);
                }
            }
        }
        None
    }

    /// The cycle an instruction can issue in. Two-uop instructions are placed
    /// conservatively: both uops must fit in the same cycle.
    pub fn instr(&mut self, tpl: &Template, start: i32, commit: bool) -> Option<i32> {
        if tpl.uop2 == PORT_NONE {
            return self.uop(tpl.uop1, start, commit);
        }
        for cycle in start.max(0)..PORT_MAP_SIZE {
            let first = self.uop(tpl.uop1, cycle, false);
            let second = self.uop(tpl.uop2, cycle, false);
            if first.is_some() && first == second {
                if commit {
                    self.uop(tpl.uop1, cycle, true);
                    self.uop(tpl.uop2, cycle, true);
                }
                return first;
            }
        }
        None
    }
}
