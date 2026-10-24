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

#[path = "../../../capsule_driver_i2c_hid/src/hid/input_mode/configure.rs"]
mod configure;
#[path = "../../../capsule_driver_i2c_hid/src/hid/input_mode/configured.rs"]
mod configured;
#[path = "../../../capsule_driver_i2c_hid/src/hid/input_mode/fallback.rs"]
mod fallback;
#[path = "../../../capsule_driver_i2c_hid/src/hid/input_mode/one_report.rs"]
mod one_report;
#[path = "../../../capsule_driver_i2c_hid/src/hid/input_mode/port.rs"]
mod port;
#[path = "../../../capsule_driver_i2c_hid/src/hid/input_mode/set_bits.rs"]
mod set_bits;
mod spi;

pub use configure::configure_reporting;
pub use spi::SpiFeatures;
