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

//! The part behind the port grant, shared by the bring-up and receive tests.

use std::{cell::Cell, rc::Rc};

use nonos_libc::{attach, Attached, Port};

use crate::constants::regs::{CMD_RESET, CMD_RX_ENABLE, CMD_TX_ENABLE, REG_CMD};

/// The register file: what is written reads back, a reset clears itself and
/// both engines unless the part is stuck, and a write at `refuse` is refused.
pub struct Part {
    pub regs: [u8; 256],
    pub stuck_in_reset: bool,
    pub refuse: Option<u16>,
    pub resets: Rc<Cell<u32>>,
}

impl Port for Part {
    fn read(&mut self, offset: u16, width: u8) -> Option<u32> {
        let at = offset as usize;
        let mut v = 0u32;
        for i in 0..width as usize {
            v |= (*self.regs.get(at + i)? as u32) << (8 * i);
        }
        Some(v)
    }

    fn write(&mut self, offset: u16, width: u8, value: u32) -> bool {
        if self.refuse == Some(offset) {
            return false;
        }
        let at = offset as usize;
        for i in 0..width as usize {
            match self.regs.get_mut(at + i) {
                Some(b) => *b = (value >> (8 * i)) as u8,
                None => return false,
            }
        }
        if offset == REG_CMD && !self.stuck_in_reset {
            if value as u8 & CMD_RESET != 0 {
                self.resets.set(self.resets.get() + 1);
                self.regs[REG_CMD as usize] &= !(CMD_RX_ENABLE | CMD_TX_ENABLE);
            }
            self.regs[REG_CMD as usize] &= !CMD_RESET;
        }
        true
    }
}

pub fn part(stuck_in_reset: bool) -> Attached {
    part_with(stuck_in_reset, None).0
}

/// The part refusing writes at `refuse`, and the resets it has done.
pub fn part_with(stuck_in_reset: bool, refuse: Option<u16>) -> (Attached, Rc<Cell<u32>>) {
    let resets = Rc::new(Cell::new(0));
    let part = Part { regs: [0; 256], stuck_in_reset, refuse, resets: Rc::clone(&resets) };
    (attach(Box::new(part)), resets)
}
