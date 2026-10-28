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

//! One register access: which block, which address, how wide. A word sits
//! on an even address and a dword on a multiple of four (the proofs hold
//! every step to that).

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Block {
    Pla,
    Usb,
}

impl Block {
    pub fn index(self) -> u16 {
        match self {
            Block::Pla => 0x0100,
            Block::Usb => 0x0000,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Width {
    W8,
    W16,
    W32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Reg {
    pub block: Block,
    pub addr: u16,
    pub width: Width,
}

const fn reg(block: Block, addr: u16, width: Width) -> Reg {
    Reg { block, addr, width }
}

pub const fn pla8(addr: u16) -> Reg {
    reg(Block::Pla, addr, Width::W8)
}
pub const fn pla16(addr: u16) -> Reg {
    reg(Block::Pla, addr, Width::W16)
}
pub const fn pla32(addr: u16) -> Reg {
    reg(Block::Pla, addr, Width::W32)
}
pub const fn usb8(addr: u16) -> Reg {
    reg(Block::Usb, addr, Width::W8)
}
pub const fn usb16(addr: u16) -> Reg {
    reg(Block::Usb, addr, Width::W16)
}
pub const fn usb32(addr: u16) -> Reg {
    reg(Block::Usb, addr, Width::W32)
}
