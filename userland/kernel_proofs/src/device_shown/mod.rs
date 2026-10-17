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

//! What MkDeviceList shows a caller of a record it does not hold, and the
//! I2C-HID record's host by device_id (K10g). The real rules are included
//! by path.

#[path = "../../../../src/hardware/broker/acpi_i2c/host_id.rs"]
pub mod host_id;
#[cfg(test)]
mod host_tests;
#[cfg(test)]
mod pci_host_tests;
#[path = "../../../../src/hardware/broker/shown.rs"]
pub mod shown;
#[cfg(test)]
mod tests;
