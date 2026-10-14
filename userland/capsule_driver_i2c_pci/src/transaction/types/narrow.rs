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

//! The reach narrowing to the touchpad once it has answered as one. A bind
//! made while the pad slept leaves every pad declared on the controller in
//! reach (setup/sequence/settle.rs). The first time a client writes exactly
//! the HID descriptor register declared for an address in reach and reads
//! a valid HID descriptor back, that address alone stays in reach for the
//! rest of the session. Nothing else narrows it, and nothing widens it.

use super::reach::Reach;

/// wHIDDescLength and bcdVersion of every HID-over-I2C descriptor.
const DESC_LEN: u16 = 30;
const BCD_1_00: u16 = 0x0100;

impl Reach {
    /// The reach once a client wrote `write` to `addr` and read `read`
    /// back: `addr` alone when `write` was its declared HID descriptor
    /// register and `read` begins as an HID descriptor; else unchanged.
    pub fn after_read(self, addr: u8, write: &[u8], read: &[u8]) -> Reach {
        let Some(reg) = self.desc_reg(addr) else { return self };
        let half = |at: usize| read.get(at..at + 2).map(|b| u16::from_le_bytes([b[0], b[1]]));
        if write == reg.to_le_bytes() && half(0) == Some(DESC_LEN) && half(2) == Some(BCD_1_00) {
            Reach::declared([(addr, reg)])
        } else {
            self
        }
    }
}
