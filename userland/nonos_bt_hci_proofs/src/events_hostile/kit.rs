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

//! Crafting HCI event frames and the little RNG the sweep shares.

use nonos_bt_wire::hci::event_code::{COMMAND_COMPLETE, COMMAND_STATUS};

pub(super) struct Rng(pub u64);
impl Rng {
    pub(super) fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    pub(super) fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
    pub(super) fn byte(&mut self) -> u8 {
        self.next() as u8
    }
}

pub(super) const SEEDS: [u64; 4] = [0x9e37_79b9_7f4a_7c15, 0x2545_f491_4f6c_dd1d, 7, 0xdead_beef];

/// An event frame: code, a stated length, then the params.
pub(super) fn frame(code: u8, stated: u8, params: &[u8]) -> Vec<u8> {
    [&[code, stated][..], params].concat()
}

/// Command Complete params: credits, the answered opcode, then returns.
pub(super) fn cc(num: u8, opcode: u16, ret: &[u8]) -> Vec<u8> {
    [&[num][..], &opcode.to_le_bytes(), ret].concat()
}

/// Command Status params: status, credits, the answered opcode.
pub(super) fn cs(status: u8, num: u8, opcode: u16) -> Vec<u8> {
    [&[status, num][..], &opcode.to_le_bytes()].concat()
}

/// A code that is neither Command Complete nor Command Status.
pub(super) fn unknown(b: u8) -> u8 {
    if b == CC || b == CS {
        0
    } else {
        b
    }
}

pub(super) const CC: u8 = COMMAND_COMPLETE;
pub(super) const CS: u8 = COMMAND_STATUS;
