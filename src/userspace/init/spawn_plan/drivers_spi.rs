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
//! The Intel LPSS SPI host's spawn (drivers_bus.rs calls it last).

/// The SPI host, after the I2C host has taken its functions; it
/// leaves at once where no function proves to be SPI.
#[cfg(feature = "nonos-capsule-driver-spi")]
pub(super) fn spawn() {
    use crate::hardware::spi_capsule as c;
    super::boot::capsule("DRIVER-SPI", "driver_spi", c::spawn_driver_spi_capsule, c::shared_state);
}

#[cfg(not(feature = "nonos-capsule-driver-spi"))]
pub(super) fn spawn() {}
