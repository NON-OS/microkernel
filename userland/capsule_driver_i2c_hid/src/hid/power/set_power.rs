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

//! SET_POWER (HID over I2C 1.0, 7.2.8): the command register, the power
//! state in the third byte's low bits, the opcode in the fourth.

use crate::i2c_client::write_read;

const OPCODE_SET_POWER: u8 = 0x08;
pub(super) const POWER_ON: u8 = 0x00;
pub(super) const POWER_SLEEP: u8 = 0x01;

/// SET_POWER to `power` through command register `reg`; whether the device
/// took it.
pub(super) fn set_power(port: u32, addr: u8, reg: u16, power: u8) -> bool {
    let r = reg.to_le_bytes();
    write_read(port, addr, &[r[0], r[1], power, OPCODE_SET_POWER], &mut []).is_some()
}
