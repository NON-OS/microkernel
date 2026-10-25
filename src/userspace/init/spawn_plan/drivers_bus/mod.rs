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

mod i2c;

pub(super) fn spawn() {
    super::drivers_wifi::spawn();
    spawn_gpio();
    i2c::spawn();
    super::drivers_smbus::spawn();
    super::drivers_spi::spawn();
}

// Before the I2C driver: the GPIO capsule claims the controllers whose
// pins the touchpad's interrupt runs through.
#[cfg(feature = "nonos-capsule-driver-gpio")]
fn spawn_gpio() {
    use crate::hardware::gpio_capsule as c;
    super::boot::capsule(
        "DRIVER-GPIO",
        "driver_gpio",
        c::spawn_driver_gpio_capsule,
        c::shared_state,
    );
}

#[cfg(not(feature = "nonos-capsule-driver-gpio"))]
fn spawn_gpio() {}
