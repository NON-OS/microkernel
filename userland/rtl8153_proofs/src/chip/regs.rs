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

//! The chip's register space as bytes. PLA 0xb000 to 0xbfff shows the PHY
//! page that PLA_OCP_GPHY_BASE selects (ocp_reg_read).

use std::collections::HashMap;

pub const PLA: u16 = 0x0100;
pub const USB: u16 = 0x0000;
/// The PHY's OCP space, a block of its own in the model.
pub const PHY: u16 = 0x0200;
const OCP_GPHY_BASE: u16 = 0xe86c;

#[derive(Default)]
pub struct Regs {
    bytes: HashMap<(u16, u16), u8>,
}

impl Regs {
    fn key(&self, ty: u16, addr: u16) -> (u16, u16) {
        if ty == PLA && (0xb000..0xc000).contains(&addr) {
            (PHY, self.word(PLA, OCP_GPHY_BASE) | (addr & 0x0fff))
        } else {
            (ty, addr)
        }
    }

    pub fn byte(&self, ty: u16, addr: u16) -> u8 {
        *self.bytes.get(&self.key(ty, addr)).unwrap_or(&0)
    }

    pub fn word(&self, ty: u16, addr: u16) -> u16 {
        u16::from_le_bytes([self.byte(ty, addr), self.byte(ty, addr + 1)])
    }

    pub fn dword(&self, ty: u16, addr: u16) -> u32 {
        u32::from_le_bytes([0, 1, 2, 3].map(|i| self.byte(ty, addr + i)))
    }

    pub fn read(&self, ty: u16, addr: u16, n: usize) -> Vec<u8> {
        (0..n as u16).map(|i| self.byte(ty, addr + i)).collect()
    }

    pub fn put(&mut self, ty: u16, addr: u16, bytes: &[u8]) {
        for (i, b) in bytes.iter().enumerate() {
            let k = self.key(ty, addr + i as u16);
            self.bytes.insert(k, *b);
        }
    }
}
