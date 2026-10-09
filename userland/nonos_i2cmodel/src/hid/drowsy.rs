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

//! A part that sleeps through its first SET_POWER. Linux i2c_hid_set_power
//! says some STM-based and Weida Tech devices need about 400 us after the
//! first clock edge to wake, so the first power-on is not acknowledged and
//! the host has to send it again.

use super::device::HidOverI2c;

/// The command opcode SET_POWER, in the low nibble of the fourth byte.
const OPCODE_SET_POWER: u8 = 0x8;

impl HidOverI2c {
    /// The same device, NACKing the opcode of the first SET_POWER it is sent.
    pub fn drowsy(mut self) -> Self {
        self.drowsy = true;
        self
    }

    /// Whether `byte`, about to be taken, is the first SET_POWER's opcode,
    /// which a drowsy device NACKs once; the command is then not carried out.
    pub(super) fn dozes_through(&mut self, byte: u8) -> bool {
        let opcode = self.written.len() == 3
            && self.pointer == Some(self.regs.command)
            && byte & 0x0F == OPCODE_SET_POWER;
        if !(self.drowsy && opcode) {
            return false;
        }
        self.drowsy = false;
        self.written.clear();
        true
    }
}
