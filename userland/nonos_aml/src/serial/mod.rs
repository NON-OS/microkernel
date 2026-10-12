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

//! Devices on serial buses (ACPI 6.5, 6.4.3.8): their I2C, SPI and UART
//! connections and GPIO pins, their _DSD properties, and HID over I2C.

mod bus;
mod bus_parse;
mod device;
mod dsd;
mod dsd_value;
mod gpio;
mod hid;
mod le;

pub use bus::{Bus, Connection};
pub use bus_parse::serial_buses;
pub use device::{serial_device, SerialDevice};
pub use dsd::{properties, DEVICE_PROPERTIES_UUID};
pub use dsd_value::{Property, Value};
pub use gpio::{gpios, Gpio};
pub use hid::{descriptor_register, is_hid_over_i2c, HID_I2C_DSM_UUID};
