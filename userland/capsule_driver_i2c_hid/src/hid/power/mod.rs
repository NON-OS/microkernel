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

//! I2C-HID power: the power-up handshake, and the sleep a device is put in
//! when it is released or given up. Real devices ship asleep: until the
//! host writes SET_POWER(ON) and RESET through the command register, most
//! touchpads never produce a single input report.

mod await_reset;
mod command_register;
mod set_power;
mod settle;
mod sleep;
mod wake;

pub use sleep::sleep;
pub use wake::wake;
