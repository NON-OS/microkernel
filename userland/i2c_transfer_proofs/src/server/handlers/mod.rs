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

//! The OP_ACPI_HID, OP_CONTROLLER_INFO, OP_GPIO_DOORBELL, OP_PROBE and
//! OP_TRANSFER handlers, as shipped.

#[path = "../../../../capsule_driver_i2c_pci/src/server/handlers/acpi_hid.rs"]
pub mod acpi_hid;
#[path = "../../../../capsule_driver_i2c_pci/src/server/handlers/controller.rs"]
pub mod controller;
#[path = "../../../../capsule_driver_i2c_pci/src/server/handlers/doorbell.rs"]
pub mod doorbell;
#[path = "../../../../capsule_driver_i2c_pci/src/server/handlers/probe.rs"]
pub mod probe;
#[path = "../../../../capsule_driver_i2c_pci/src/server/handlers/transfer.rs"]
pub mod transfer;
