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

//! The driver's live state and its PCI command value, without the broker
//! sequence that builds them. The tests build a driver over host memory and
//! a window instead.

#[path = "../../../capsule_driver_igc/src/setup/driver.rs"]
mod driver;
#[path = "../../../capsule_driver_igc/src/setup/pci_command.rs"]
pub mod pci_command;

pub use driver::Driver;
