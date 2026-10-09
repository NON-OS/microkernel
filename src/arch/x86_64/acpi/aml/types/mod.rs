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

//! The records the bounded AML extractor returns, one file per record.

mod gpio_controller;
mod i2c_hid_device;
mod lpss_controller;

pub use gpio_controller::{GpioController, GPIO_MAX_WINDOWS};
pub use i2c_hid_device::I2cHidDevice;
pub use lpss_controller::LpssController;
