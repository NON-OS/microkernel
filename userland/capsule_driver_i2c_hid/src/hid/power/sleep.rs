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

//! SET_POWER(SLEEP): the low-power state the specification gives a device
//! the host is not reading. It still answers on the bus, so SET_POWER(ON)
//! wakes it (wake.rs).

use super::command_register::command_register;
use super::set_power::{set_power, POWER_SLEEP};

/// Tell the device to sleep; false when it names no command register or
/// refused the command.
pub fn sleep(port: u32, addr: u8, desc: &[u8; 30]) -> bool {
    let reg = command_register(desc);
    reg != 0 && set_power(port, addr, reg, POWER_SLEEP)
}
