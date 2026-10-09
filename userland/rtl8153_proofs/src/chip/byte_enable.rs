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

//! A register write as the chip takes it, under the byte enables of its
//! request the way generic_ocp_write's splitting implies: the first
//! dword's bytes by the low nibble, the last dword's by the high nibble,
//! the dwords between whole.

use super::regs::Regs;

impl Regs {
    pub fn write(&mut self, ty: u16, addr: u16, byen: u8, data: &[u8]) {
        let last = data.len().saturating_sub(1) / 4;
        for (i, b) in data.iter().enumerate() {
            let mask = match i / 4 {
                0 => byen & 0x0f,
                d if d == last => byen >> 4,
                _ => 0x0f,
            };
            if mask & (1 << (i % 4)) != 0 {
                self.put(ty, addr + i as u16, &[*b]);
            }
        }
    }
}
