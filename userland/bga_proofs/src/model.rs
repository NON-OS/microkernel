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

//! A DISPI adapter recording every access in order. Registers read back what
//! was written, but a `stuck` one keeps its value, as QEMU clamps a mode.

use crate::constants::QEXT_SIZE;
use crate::hw::Hw;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Op {
    Dispi(u32, u16),
    VgaRead(u16),
    Vga(u16, u8),
    Qext(u32, u32),
}

#[derive(Default)]
pub struct Model {
    pub regs: std::cell::RefCell<[u16; 16]>,
    pub log: std::cell::RefCell<Vec<Op>>,
    pub qext_size: u32,
    pub stuck: Option<(u32, u16)>,
}

impl Model {
    pub fn with(regs: &[(u32, u16)]) -> Self {
        let m = Model::default();
        for &(i, v) in regs {
            m.regs.borrow_mut()[i as usize] = v;
        }
        m
    }
}

impl Hw for Model {
    fn dispi(&self, index: u32) -> u16 {
        self.regs.borrow()[index as usize]
    }
    fn set_dispi(&self, index: u32, value: u16) {
        self.log.borrow_mut().push(Op::Dispi(index, value));
        let kept = self.stuck.filter(|&(i, _)| i == index).map(|(_, v)| v);
        self.regs.borrow_mut()[index as usize] = kept.unwrap_or(value);
    }
    fn vga(&self, port: u16) -> u8 {
        self.log.borrow_mut().push(Op::VgaRead(port));
        0
    }
    fn set_vga(&self, port: u16, value: u8) {
        self.log.borrow_mut().push(Op::Vga(port, value));
    }
    fn qext(&self, off: u32) -> u32 {
        match off {
            QEXT_SIZE => self.qext_size,
            _ => 0,
        }
    }
    fn set_qext(&self, off: u32, value: u32) {
        self.log.borrow_mut().push(Op::Qext(off, value));
    }
}
